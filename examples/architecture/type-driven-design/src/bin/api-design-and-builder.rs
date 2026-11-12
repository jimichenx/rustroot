//! Lesson 15.4: Idiomatic API signature design without function overloading or default args.

// ANCHOR: full
// ANCHOR: lightweight_patterns
#[derive(Debug)]
struct RetryPolicy {
    max_retries: u8,
    backoff_ms: u64,
    jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            backoff_ms: 200,
            jitter: true,
        }
    }
}

impl RetryPolicy {
    // 1. Named constructors instead of overloaded `new(...)`
    fn new() -> Self {
        Self::default()
    }

    fn with_retries(max_retries: u8) -> Self {
        Self {
            max_retries,
            ..Self::default()
        }
    }
}

// 2. Generic `AsRef<str>` for read-only multi-type input
fn ping_host<S: AsRef<str>>(target: S) -> usize {
    target.as_ref().len()
}

// 3. Slice `&[u16]` for variable-length homogeneous arguments
fn allow_ports(ports: &[u16]) -> usize {
    ports.len()
}
// ANCHOR_END: lightweight_patterns

// ANCHOR: builder_pattern
pub mod server {
    #[derive(Debug)]
    pub struct ServerConfig {
        host: String,
        port: u16,
        max_connections: usize,
        enable_tls: bool,
    }

    impl ServerConfig {
        pub fn summary(&self) -> String {
            format!(
                "{}:{} (max_conn={}, tls={})",
                self.host, self.port, self.max_connections, self.enable_tls
            )
        }
    }

    pub struct ServerConfigBuilder {
        host: String,
        port: u16,
        max_connections: usize,
        enable_tls: bool,
    }

    impl ServerConfigBuilder {
        // Required argument in `new`, safe defaults for optional fields
        pub fn new(host: impl Into<String>) -> Self {
            Self {
                host: host.into(),
                port: 8080,
                max_connections: 1024,
                enable_tls: false,
            }
        }

        // Consume and return `Self` by value for one-expression chaining
        pub fn port(mut self, port: u16) -> Self {
            self.port = port;
            self
        }

        pub fn max_connections(mut self, max: usize) -> Self {
            self.max_connections = max;
            self
        }

        pub fn enable_tls(mut self, tls: bool) -> Self {
            self.enable_tls = tls;
            self
        }

        pub fn build(self) -> ServerConfig {
            ServerConfig {
                host: self.host,
                port: self.port,
                max_connections: self.max_connections,
                enable_tls: self.enable_tls,
            }
        }
    }
}
// ANCHOR_END: builder_pattern

fn main() {
    let p1 = RetryPolicy::new();
    let p2 = RetryPolicy::with_retries(5);
    let p3 = RetryPolicy {
        backoff_ms: 500,
        ..Default::default()
    };
    println!("policies: {:?} | {:?} | jitter={}", p1.max_retries, p2.max_retries, p3.jitter && p3.backoff_ms == 500);

    println!(
        "ping len = {}, ports = {}",
        ping_host("api.rustroot.cn") + ping_host(String::from("127.0.0.1")),
        allow_ports(&[80, 443, 8443]),
    );

    let cfg = server::ServerConfigBuilder::new("0.0.0.0")
        .port(9443)
        .enable_tls(true)
        .build();
    println!("server = {}", cfg.summary());
}
// ANCHOR_END: full
