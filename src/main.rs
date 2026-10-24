//! The `balanse` command.

use std::process::ExitCode;

use anyhow::Context;

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
    match std::env::args().nth(1).as_deref() {
        Some("migrate") => {
            let url = std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
            let pool = sqlx::PgPool::connect(&url)
                .await
                .context("connecting to the database")?;
            balanse::MIGRATOR.run(&pool).await.context("running migrations")?;
            println!("migrations applied");
            Ok(())
        }
        _ => anyhow::bail!("usage: balanse migrate"),
    }
}
