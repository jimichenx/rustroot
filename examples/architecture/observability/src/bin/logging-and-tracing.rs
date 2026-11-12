//! Lesson 17.2: Structured observability with `tracing`, async `Span` propagation, and single-boundary error logging.

// ANCHOR: full
use anyhow::{Context, Result};
use tracing::{error, info, warn};

// ANCHOR: instrumented_async
// `skip(api_token)` prevents sensitive credentials from ever being recorded into spans/logs
#[tracing::instrument(name = "order_flow", skip(api_token))]
async fn process_order(order_id: u64, amount_cents: u64, api_token: &str) -> Result<()> {
    info!(amount_cents, "starting async order pipeline");

    // Yield across `.await`: in a multi-threaded runtime the task may resume on another thread,
    // yet the `order_flow{order_id=...}` span stays attached to this Future's poll lifecycle.
    tokio::task::yield_now().await;

    audit_amount(amount_cents)?;
    let _ = api_token.len();
    info!("order settled");
    Ok(())
}

fn audit_amount(amount_cents: u64) -> Result<()> {
    if amount_cents >= 100_000 {
        // Notice: `audit_amount` does not take `order_id`, yet inherits the active Span context!
        warn!(amount_cents, "high-value order flagged for review");
    }
    if amount_cents == 0 {
        // Inner layer NEVER logs `error!`; it only returns a contextualized error upward.
        anyhow::bail!("zero-amount order rejected");
    }
    Ok(())
}
// ANCHOR_END: instrumented_async

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Only the binary entrypoint initializes the global subscriber once
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_ansi(false)
        .init();

    // Concurrent execution of two orders: spans keep each request's causal context isolated
    let (r1, r2) = tokio::join!(
        process_order(1001, 250_000, "secret-token-a"),
        process_order(1002, 0, "secret-token-b"),
    );

    // Top boundary rule: log unhandled errors exactly ONCE at the outer boundary
    for (order_id, res) in [(1001, r1), (1002, r2)] {
        if let Err(err) = res.with_context(|| format!("request aborted for order #{order_id}")) {
            error!(error = %format!("{err:#}"), "top-level handler caught failure");
        }
    }
}
// ANCHOR_END: full
