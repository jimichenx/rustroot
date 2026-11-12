//! Additive Cargo features and conditional compilation.
//! Try:
//!   cargo run -p engineering-tools --bin cargo-features
//!   cargo run -p engineering-tools --bin cargo-features --all-features
//!   cargo run -p engineering-tools --bin cargo-features --no-default-features

// ANCHOR: full
#[cfg(feature = "json")]
fn encode_json(payload: &str) -> String {
    format!("{{\"data\":\"{payload}\"}}")
}

#[cfg(feature = "compression")]
fn compress_tag() -> &'static str {
    "gzip"
}

fn active_capabilities() -> Vec<&'static str> {
    let mut caps = Vec::new();
    if cfg!(feature = "json") {
        caps.push("json");
    }
    if cfg!(feature = "compression") {
        caps.push("compression");
    }
    caps
}

fn main() {
    println!("enabled features: {:?}", active_capabilities());

    #[cfg(feature = "json")]
    println!("encoded: {}", encode_json("ok"));

    #[cfg(feature = "compression")]
    println!("encoding header: {}", compress_tag());
}
// ANCHOR_END: full
