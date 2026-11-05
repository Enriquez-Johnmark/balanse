//! G6: the Owner's payment instructions show on the Debtor's page, and
//! changing them needs a fresh login.

#![allow(clippy::unwrap_used, reason = "test code: a failed unwrap is a failed test")]

mod common;

use common::{app, business, call, clock, log_in};
use serde_json::json;
use sqlx::PgPool;
use time::Duration;

#[sqlx::test(migrator = "balanse::MIGRATOR")]
async fn g6_payment_instructions_change_only_soon_after_logging_in(pool: PgPool) {
    let clock = clock();
    business(&pool, &clock).await;
    let app = app(&pool, &clock);
    let ana = log_in(&app, &pool, "ana@reyes.test").await;
    let gcash = json!({ "payment_instructions": "GCash 0917 123 4567 (Ana Reyes)" });

    let fresh = call(
        &app,
        "PUT",
        "/api/settings/payment-instructions",
        Some(gcash),
        Some(&ana),
    )
    .await;
    let shown = call(&app, "GET", "/api/settings", None, Some(&ana)).await;
    clock.advance(Duration::minutes(11));
    let swap = json!({ "payment_instructions": "BDO 0000 1111 2222 (someone else)" });
    let stale = call(
        &app,
        "PUT",
        "/api/settings/payment-instructions",
        Some(swap),
        Some(&ana),
    )
    .await;
    let after = call(&app, "GET", "/api/settings", None, Some(&ana)).await;

    assert_eq!(fresh.status.as_u16(), 204);
    assert_eq!(shown.body["payment_instructions"], "GCash 0917 123 4567 (Ana Reyes)");
    assert_eq!(stale.status.as_u16(), 403);
    assert!(
        stale.body["message"].as_str().unwrap().contains("Log in again"),
        "{}",
        stale.body
    );
    assert_eq!(
        after.body["payment_instructions"], "GCash 0917 123 4567 (Ana Reyes)",
        "unchanged"
    );
    let logged: i64 =
        sqlx::query_scalar("select count(*) from audit_events where action = 'business.payment_instructions_changed'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(logged, 1);
}
