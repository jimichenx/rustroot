// ANCHOR: full
use std::mem;

// ANCHOR: port_contracts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Port(u16);

// Hand-written `Default` preserves the `1..=65535` invariant (`#[derive(Default)]` would yield illegal `Port(0)`).
impl Default for Port {
    fn default() -> Self {
        Port(8080)
    }
}

impl TryFrom<u32> for Port {
    type Error = &'static str;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        if (1..=65535).contains(&raw) {
            Ok(Port(raw as u16))
        } else {
            Err("port out of 1..=65535 range")
        }
    }
}

impl From<Port> for u16 {
    fn from(port: Port) -> Self {
        port.0
    }
}
// ANCHOR_END: port_contracts

// ANCHOR: to_owned_and_take
fn build_authority(host: &str, port: Option<Port>) -> String {
    let resolved = port.unwrap_or_default();
    // `str` is unsized (`!Sized`), so `ToOwned` upgrades `&str` into heap-owned `String`
    let mut authority: String = host.to_owned();
    authority.push_str(&format!(":{}", u16::from(resolved)));
    authority
}

fn flush_buffer(buf: &mut String) -> Vec<u8> {
    // `mem::take` swaps in `String::default()` (zero heap alloc) and extracts ownership
    let taken: String = mem::take(buf);
    taken.into_bytes()
}
// ANCHOR_END: to_owned_and_take

fn main() {
    println!("default = {}", build_authority("localhost", None));
    println!(
        "try_from: ok={:?}, err={:?}",
        Port::try_from(9000),
        Port::try_from(0),
    );

    let mut pending = String::from("PING");
    let peek_slice: &[u8] = pending.as_bytes();
    println!("as_bytes len = {}", peek_slice.len());

    let wire_bytes: Vec<u8> = flush_buffer(&mut pending);
    println!("into_bytes = {:?}, after take len = {}", wire_bytes, pending.len());
}
// ANCHOR_END: full
