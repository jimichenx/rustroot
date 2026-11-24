use std::mem::size_of;

// ANCHOR: closed_enum
enum PaymentMethod {
    Cash,
    Card { last4: u16 },
    Wallet { provider: &'static str },
}

impl PaymentMethod {
    fn pay(&self, amount_cents: u64) -> String {
        match self {
            PaymentMethod::Cash => format!("paid {amount_cents}c in cash"),
            PaymentMethod::Card { last4 } => format!("charged {amount_cents}c to card *{last4}"),
            PaymentMethod::Wallet { provider } => format!("paid {amount_cents}c via {provider}"),
        }
    }
}
// ANCHOR_END: closed_enum

// ANCHOR: open_trait
trait Notifier {
    fn send(&self, msg: &str) -> String;
}

struct EmailNotifier {
    addr: &'static str,
}

struct WebhookNotifier {
    url: &'static str,
}

impl Notifier for EmailNotifier {
    fn send(&self, msg: &str) -> String {
        format!("Email to {}: {msg}", self.addr)
    }
}

impl Notifier for WebhookNotifier {
    fn send(&self, msg: &str) -> String {
        format!("POST {}: {msg}", self.url)
    }
}

fn broadcast(channels: &[&dyn Notifier], msg: &str) {
    for ch in channels {
        println!("{}", ch.send(msg));
    }
}
// ANCHOR_END: open_trait

fn main() {
    let payments = [
        PaymentMethod::Cash,
        PaymentMethod::Card { last4: 4242 },
        PaymentMethod::Wallet { provider: "Alipay" },
    ];
    println!("size of PaymentMethod = {} bytes (stack-inlined)", size_of::<PaymentMethod>());
    for p in &payments {
        println!("{}", p.pay(1500));
    }

    let email = EmailNotifier { addr: "ops@example.com" };
    let hook = WebhookNotifier { url: "https://hooks.internal/alert" };
    broadcast(&[&email, &hook], "deploy finished");
}
