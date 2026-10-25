//! The `balanse` command.

use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use balanse::balanse::account_emails::welcome_email;
use balanse::balanse::debtor_links::DebtorLinks;
use balanse::balanse::presend::BalanseChecks;
use balanse::balanse::reminders::{MANILA, ReplyAddresses, Scheduler};
use balanse::balanse::replies::Replies;
use balanse::balanse::signups::approve_signup;
use balanse::balanse::web::{AppState, router};
use balanse::config::{Config, MailConfig};
use balanse::engine::accounts::{LoginLink, Logins, RequestId, Signups};
use balanse::engine::clock::{Clock, FixedClock, SystemClock};
use balanse::engine::outbox::Sender;
use balanse::engine::signed::Signer;

/// How many messages one drain may send.
const DRAIN_BATCH: usize = 50;

const USAGE: &str = "usage: balanse serve | migrate | outbox-drain | schedule | signups | approve-signup <id> | \
                     decline-signup <id> | dev-schedule --date YYYY-MM-DD";
/// How often `serve` drains the outbox.
const DRAIN_EVERY: Duration = Duration::from_secs(30);
/// How often `serve` drafts the reminders that are due.
const SCHEDULE_EVERY: Duration = Duration::from_secs(60 * 60);

fn main() -> ExitCode {
    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("starting the async runtime")
        .and_then(|runtime| runtime.block_on(run()));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("balanse: {err:#}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let command = std::env::args().nth(1);
    let Some(command) = command.as_deref() else {
        anyhow::bail!(USAGE);
    };
    let config = Config::from_env().context("reading the settings")?;
    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .context("connecting to the database")?;
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    match command {
        "serve" => serve(&config, pool, clock).await?,
        "migrate" => {
            balanse::MIGRATOR.run(&pool).await.context("running migrations")?;
            println!("migrations applied");
        }
        "outbox-drain" => {
            let links = debtor_links(&config, &pool, &clock)?;
            let checks = Arc::new(BalanseChecks::for_sending(pool.clone(), clock.clone(), links));
            let mut sender = Sender::new(pool, config.mailer()?, clock, checks);
            if let Some(budget) = config.send_budget() {
                sender = sender.with_budget(budget);
            }
            let report = sender.drain_once(DRAIN_BATCH).await.context("sending")?;
            println!("{report:?}");
        }
        "schedule" => {
            let links = debtor_links(&config, &pool, &clock)?;
            let report = Scheduler::new(pool, clock, reply_addresses(&config)?, links)
                .run()
                .await
                .context("drafting")?;
            println!("{report:?}");
        }
        "signups" => {
            for request in Signups::new(pool, clock).pending().await? {
                println!(
                    "{}  {}  {}  {}",
                    request.id,
                    request.requested_at.date(),
                    request.name,
                    request.email
                );
            }
        }
        "approve-signup" => {
            let id = request_id()?;
            let business = approve_signup(&pool, clock.clone(), id).await?;
            let base_url = config.public_base_url.clone();
            let welcome = |link: &LoginLink<'_>| welcome_email(&base_url, link);
            let owner = balanse::engine::accounts::Accounts::new(pool.clone(), clock.clone())
                .get(business)
                .await?
                .owner_email;
            let checks = BalanseChecks::for_approval(pool.clone(), debtor_links(&config, &pool, &clock)?);
            Logins::new(pool, clock).send_link(&owner, welcome, &checks).await?;
            println!("approved {business}; a welcome link is on its way to {owner}");
        }
        "decline-signup" => {
            Signups::new(pool, clock).decline(request_id()?).await?;
            println!("declined");
        }
        "dev-schedule" => dev_schedule(&config, pool).await?,
        _ => anyhow::bail!(USAGE),
    }
    Ok(())
}

/// The request ID given after the command.
fn request_id() -> anyhow::Result<RequestId> {
    let text = std::env::args().nth(2).context(USAGE)?;
    let id: uuid::Uuid = text.parse().context("not a request ID")?;
    Ok(RequestId::from(id))
}

/// The signed links to Debtor pages that reminders carry.
fn debtor_links(config: &Config, pool: &sqlx::PgPool, clock: &Arc<dyn Clock>) -> anyhow::Result<DebtorLinks> {
    Ok(DebtorLinks::new(
        pool.clone(),
        clock.clone(),
        &config.master_key()?,
        &config.public_base_url,
    ))
}

/// The signed reply addresses reminders carry.
fn reply_addresses(config: &Config) -> anyhow::Result<ReplyAddresses> {
    let signer = Arc::new(Signer::new(&config.master_key()?, "balanse reply address v1"));
    Ok(ReplyAddresses::new(signer, &config.reply_domain))
}

/// `balanse dev-schedule --date YYYY-MM-DD` (development only): drafts the
/// reminders due on that date, as if it were 07:30 there in Manila, so the
/// whole loop can be tried before real dates come round. They are then
/// approved from the Drafts page as usual.
async fn dev_schedule(config: &Config, pool: sqlx::PgPool) -> anyhow::Result<()> {
    if !matches!(config.mail, MailConfig::Mailpit { .. }) {
        anyhow::bail!("dev-schedule is for development only and runs only with MAIL_PROVIDER=mailpit");
    }
    let date = match std::env::args().skip(2).collect::<Vec<_>>().as_slice() {
        [flag, date] if flag == "--date" => {
            time::Date::parse(date, time::macros::format_description!("[year]-[month]-[day]")).ok()
        }
        _ => None,
    };
    let Some(date) = date else {
        anyhow::bail!("usage: balanse dev-schedule --date YYYY-MM-DD");
    };
    balanse::MIGRATOR.run(&pool).await.context("running migrations")?;
    let clock: Arc<dyn Clock> = Arc::new(FixedClock::new(date.with_hms(7, 30, 0)?.assume_offset(MANILA)));
    let links = debtor_links(config, &pool, &clock)?;
    let report = Scheduler::new(pool, clock, reply_addresses(config)?, links)
        .run()
        .await
        .context("drafting")?;
    println!(
        "{} reminder draft(s) created as of {date}; approve them on the Drafts page.",
        report.drafted
    );
    Ok(())
}

/// Runs the web server, the outbox sender and the scheduler until stopped.
async fn serve(config: &Config, pool: sqlx::PgPool, clock: Arc<dyn Clock>) -> anyhow::Result<()> {
    balanse::MIGRATOR.run(&pool).await.context("running migrations")?;
    let addresses = reply_addresses(config)?;
    let links = debtor_links(config, &pool, &clock)?;
    let mut state = AppState::new(
        pool.clone(),
        clock.clone(),
        &config.master_key()?,
        &config.public_base_url,
    )
    .with_web_dir(&config.web_dir);
    if let Some(secret) = &config.inbound_secret {
        let replies = Replies::new(pool.clone(), clock.clone(), addresses.clone(), links.clone());
        state = state.with_inbound(secret.clone(), replies);
    } else {
        tracing::warn!("INBOUND_SECRET isn't set: replies aren't taken in");
    }
    let wake = state.sender_wake_up();

    let checks = Arc::new(BalanseChecks::for_sending(pool.clone(), clock.clone(), links.clone()));
    let mut sender = Sender::new(pool.clone(), config.mailer()?, clock.clone(), checks);
    if let Some(budget) = config.send_budget() {
        sender = sender.with_budget(budget);
    }
    tokio::spawn(async move {
        loop {
            if let Err(err) = sender.drain_once(DRAIN_BATCH).await {
                tracing::error!(%err, "sending failed");
            }
            // Wakes early when an email is approved.
            let _ = tokio::time::timeout(DRAIN_EVERY, wake.notified()).await;
        }
    });
    let scheduler = Scheduler::new(pool, clock, addresses, links);
    tokio::spawn(async move {
        loop {
            if let Err(err) = scheduler.run().await {
                tracing::error!(%err, "drafting reminders failed");
            }
            tokio::time::sleep(SCHEDULE_EVERY).await;
        }
    });

    let listener = tokio::net::TcpListener::bind(&config.bind_addr)
        .await
        .with_context(|| format!("listening on {}", config.bind_addr))?;
    tracing::info!(addr = %config.bind_addr, "serving");
    axum::serve(listener, router(state))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .context("serving")?;
    Ok(())
}
