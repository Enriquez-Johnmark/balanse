//! The `balanse` command.

use std::process::ExitCode;
use std::sync::Arc;

use anyhow::Context;
use balanse::balanse::account_emails::welcome_email;
use balanse::balanse::presend::BalanseChecks;
use balanse::balanse::reminders::{ReplyAddresses, Scheduler};
use balanse::balanse::signups::approve_signup;
use balanse::config::Config;
use balanse::engine::accounts::{LoginLink, Logins, RequestId, Signups};
use balanse::engine::clock::{Clock, SystemClock};
use balanse::engine::outbox::Sender;
use balanse::engine::signed::Signer;

/// How many messages one drain may send.
const DRAIN_BATCH: usize = 50;

const USAGE: &str =
    "usage: balanse migrate | outbox-drain | schedule | signups | approve-signup <id> | decline-signup <id>";

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
        "migrate" => {
            balanse::MIGRATOR.run(&pool).await.context("running migrations")?;
            println!("migrations applied");
        }
        "outbox-drain" => {
            let checks = Arc::new(BalanseChecks::for_sending(pool.clone(), clock.clone()));
            let mut sender = Sender::new(pool, config.mailer()?, clock, checks);
            if let Some(budget) = config.send_budget() {
                sender = sender.with_budget(budget);
            }
            let report = sender.drain_once(DRAIN_BATCH).await.context("sending")?;
            println!("{report:?}");
        }
        "schedule" => {
            let signer = Arc::new(Signer::new(&config.master_key()?, "balanse reply address v1"));
            let replies = ReplyAddresses::new(signer, &config.reply_domain);
            let report = Scheduler::new(pool, clock, replies).run().await.context("drafting")?;
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
            Logins::new(pool.clone(), clock)
                .send_link(&owner, welcome, &BalanseChecks::for_approval(pool))
                .await?;
            println!("approved {business}; a welcome link is on its way to {owner}");
        }
        "decline-signup" => {
            Signups::new(pool, clock).decline(request_id()?).await?;
            println!("declined");
        }
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
