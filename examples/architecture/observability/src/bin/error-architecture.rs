//! Lesson 17.1: Layered error handling with `thiserror` (domain library) and `anyhow` (application boundary).

// ANCHOR: full
// ANCHOR: domain_thiserror
pub mod billing {
    use std::io;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum BillingError {
        #[error("insufficient funds: balance {balance}, required {required}")]
        InsufficientFunds { balance: u64, required: u64 },

        #[error("payment gateway I/O failure")]
        GatewayIo(#[from] io::Error),
    }

    pub fn charge(balance: u64, required: u64, gateway_down: bool) -> Result<u64, BillingError> {
        if gateway_down {
            // `?` automatically converts `io::Error` into `BillingError::GatewayIo` via `#[from]`
            return Err(io::Error::new(io::ErrorKind::TimedOut, "tcp connect timeout"))?;
        }
        if balance < required {
            return Err(BillingError::InsufficientFunds { balance, required });
        }
        Ok(balance - required)
    }
}
// ANCHOR_END: domain_thiserror

// ANCHOR: app_anyhow
use anyhow::{Context, Result, ensure};
use billing::BillingError;

fn handle_checkout(user_id: u64, order_id: u64, required: u64, gateway_down: bool) -> Result<String> {
    ensure!(user_id > 0, "anonymous user is not allowed to checkout");

    match billing::charge(500, required, gateway_down) {
        Ok(rem) => Ok(format!("order #{order_id} paid, remaining = {rem}")),
        // Domain caller can still pattern-match specific structured variants when recoverable:
        Err(BillingError::InsufficientFunds { balance, required }) => {
            Ok(format!("order #{order_id} downgraded to unpaid draft ({balance} < {required})"))
        }
        // Unexpected infrastructure failures bubble up with business context attached:
        Err(other) => Err(other)
            .with_context(|| format!("checkout failed for user #{user_id}, order #{order_id}")),
    }
}
// ANCHOR_END: app_anyhow

fn main() {
    // 1. Recoverable domain variant handled by pattern matching
    println!("{}", handle_checkout(42, 1001, 800, false).unwrap());

    // 2. Unhandled infrastructure failure surfaces full causal chain at top boundary
    let err = handle_checkout(42, 1002, 300, true).unwrap_err();
    println!("top error: {err}");
    for (idx, cause) in err.chain().enumerate() {
        println!("  cause[{idx}]: {cause}");
    }
}
// ANCHOR_END: full
