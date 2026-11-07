//! Four idiomatic Rust patterns that replace traditional function overloading
//! and default parameters:
//! 1. Named constructors (`new`, `with_timeout`, `from_parts`)
//! 2. Generic conversion bounds (`Into<T>` / `AsRef<T>`)
//! 3. Config struct + `Default` struct update syntax (`..Default::default()`)
//! 4. Slices (`&[T]`) for variable-length homogeneous arguments

#[derive(Debug)]
struct ClientConfig {
    host: &'static str,
    port: u16,
    timeout_ms: u64,
    max_retries: u8,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            host: "127.0.0.1",
            port: 8080,
            timeout_ms: 3000,
            max_retries: 3,
        }
    }
}

impl ClientConfig {
    // 1. Named constructors instead of overloaded `new(...)`:
    fn new(host: &'static str) -> Self {
        ClientConfig { host, ..Default::default() }
    }

    fn with_port(host: &'static str, port: u16) -> Self {
        ClientConfig { host, port, ..Default::default() }
    }
    fn summary(&self) -> String {
        format!("{}:{} (timeout={}ms, retries={})", self.host, self.port, self.timeout_ms, self.max_retries)
    }
}

// 2. Generic `AsRef<str>` instead of overloading for `&str` and `String`:
fn ping<S: AsRef<str>>(target: S) {
    println!("pinging {}", target.as_ref());
}

// 4. Slice `&[u16]` instead of variadic arguments:
fn check_ports(ports: &[u16]) -> usize {
    ports.len()
}

fn main() {
    let c1 = ClientConfig::new("example.com");
    let c2 = ClientConfig::with_port("example.com", 443);
    // 3. Struct update syntax with `..Default::default()` for optional args:
    let c3 = ClientConfig {
        host: "api.internal",
        timeout_ms: 500,
        ..Default::default()
    };
    println!("c1 = {}\nc2 = {}\nc3 = {}", c1.summary(), c2.summary(), c3.summary());

    ping("localhost");
    ping(String::from("10.0.0.1"));
    println!("checked {} ports", check_ports(&[80, 443, 8080]));
}
