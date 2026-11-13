// ANCHOR: full
use std::borrow::Cow;
use serde::Deserialize;

// ANCHOR: validated_newtype
pub mod config {
    use serde::Deserialize;

    #[derive(Debug, Clone, Copy, Deserialize)]
    #[serde(try_from = "u16")]
    pub struct ListenPort(u16);

    impl ListenPort {
        pub fn get(self) -> u16 {
            self.0
        }
    }

    impl TryFrom<u16> for ListenPort {
        type Error = &'static str;

        fn try_from(raw: u16) -> Result<Self, Self::Error> {
            if raw == 0 {
                return Err("listen port must be in 1..=65535");
            }
            Ok(Self(raw))
        }
    }

    fn default_workers() -> usize {
        4
    }

    #[derive(Debug, Deserialize)]
    pub struct AppConfig {
        pub listen_port: ListenPort,
        #[serde(default = "default_workers")]
        pub workers: usize,
    }
}
// ANCHOR_END: validated_newtype

// ANCHOR: tagged_enum_and_zero_copy
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum WebhookEvent<'a> {
    OrderPaid {
        order_id: u64,
        #[serde(borrow)]
        note: Cow<'a, str>,
    },
    OrderRefunded {
        order_id: u64,
        reason: String,
    },
}
// ANCHOR_END: tagged_enum_and_zero_copy

fn main() {
    let bad_cfg = r#"{"listen_port": 0}"#;
    let err = serde_json::from_str::<config::AppConfig>(bad_cfg).unwrap_err();
    println!("rejected invalid config: {err}");

    let ok_cfg = r#"{"listen_port": 8080}"#;
    let cfg: config::AppConfig = serde_json::from_str(ok_cfg).unwrap();
    println!("loaded config: port={}, workers={}", cfg.listen_port.get(), cfg.workers);

    let raw_event = r#"{"type": "order_paid", "order_id": 1001, "note": "vip-fast-track"}"#;
    let event: WebhookEvent<'_> = serde_json::from_str(raw_event).unwrap();
    if let WebhookEvent::OrderPaid { order_id, note } = event {
        let borrowed = matches!(note, Cow::Borrowed(_));
        println!("event order #{order_id}: note={note}, zero_copy_borrowed={borrowed}");
    }

    let refund_json = r#"{"type": "order_refunded", "order_id": 1002, "reason": "duplicate"}"#;
    let refund: WebhookEvent<'_> = serde_json::from_str(refund_json).unwrap();
    if let WebhookEvent::OrderRefunded { order_id, reason } = refund {
        println!("event refund #{order_id}: reason={reason}");
    }
}
// ANCHOR_END: full
