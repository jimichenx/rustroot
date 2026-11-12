//! Lesson 7.5: Ecosystem convention traits matrix and `as_` / `to_` / `into_` cost rules.

// ANCHOR: full
// ANCHOR: port_traits
struct Port(u16);

// 1. Explicit zero-arg default value (`Default`)
impl Default for Port {
    fn default() -> Self {
        Port(8080)
    }
}

// 2. Infallible consuming conversion (`From` unlocks `Into`)
impl From<u16> for Port {
    fn from(raw: u16) -> Self {
        Port(raw)
    }
}

// 3. Fallible consuming conversion (`TryFrom` uses associated type `Error`)
impl TryFrom<u32> for Port {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 || value > 65535 {
            Err("port out of 1..=65535 range")
        } else {
            Ok(Port(value as u16))
        }
    }
}
// ANCHOR_END: port_traits

// ANCHOR: as_ref_and_owned
// 4. Zero-allocation borrowed view (`AsRef<str>`)
fn format_endpoint<S: AsRef<str>>(host: S, port: Port) -> String {
    // 5. Borrowed unsized `str` upgrades to heap-owned `String` via `ToOwned`
    let mut owned_host: String = host.as_ref().to_owned();
    owned_host.push_str(&format!(":{}", port.0));
    owned_host
}
// ANCHOR_END: as_ref_and_owned

fn main() {
    let fallback: Option<Port> = None;
    let default_ep = format_endpoint("localhost", fallback.unwrap_or_default());
    let prod_ep = format_endpoint(String::from("10.0.0.8"), 443u16.into());
    println!("{default_ep} | {prod_ep}");

    println!(
        "valid = {:?}, invalid = {:?}",
        Port::try_from(9000u32).map(|p| p.0),
        Port::try_from(70000u32).map(|p| p.0),
    );

    // `as_` (zero-alloc borrow) vs `into_` (ownership transfer, zero heap copy)
    let token = String::from("ok");
    let borrowed_bytes: &[u8] = token.as_bytes();
    println!("borrowed len = {}", borrowed_bytes.len());
    let owned_bytes: Vec<u8> = token.into_bytes();
    println!("reused heap buffer = {owned_bytes:?}");
}
// ANCHOR_END: full
