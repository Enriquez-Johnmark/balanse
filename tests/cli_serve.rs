//! `balanse serve` starts the web server and answers its health check.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

use std::process::{Command, Stdio};
use std::time::Duration;

#[tokio::test]
async fn serve_answers_the_health_check() {
    // A free port, released for the server to take.
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let mut server = Command::new(env!("CARGO_BIN_EXE_balanse"))
        .arg("serve")
        .env("BIND_ADDR", format!("127.0.0.1:{port}"))
        .env("PUBLIC_BASE_URL", format!("http://127.0.0.1:{port}"))
        .env("MASTER_KEY", "ab".repeat(32))
        .env("RUST_LOG", "warn")
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    let mut answer = None;
    for _ in 0..100 {
        if let Ok(response) = reqwest::get(format!("http://127.0.0.1:{port}/health")).await {
            answer = Some(response.text().await.unwrap());
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    server.kill().unwrap();
    server.wait().unwrap();

    assert_eq!(answer.as_deref(), Some("ok"));
}
