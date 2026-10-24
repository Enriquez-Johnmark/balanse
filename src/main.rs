//! The `balanse` command.

use std::process::ExitCode;
use std::sync::Arc;

use anyhow::Context;
use balanse::balanse::presend::BalanseChecks;
use balanse::balanse::reminders::{ReplyAddresses, Scheduler};
use balanse::config::Config;
use balanse::engine::clock::{Clock, SystemClock};
use balanse::engine::outbox::Sender;
use balanse::engine::signed::Signer;

/// How many messages one drain may send.
const DRAIN_BATCH: usize = 50;

const USAGE: &str = "usage: balanse migrate | outbox-drain | schedule";

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
        _ => anyhow::bail!(USAGE),
    }
    Ok(())
}
