// ANCHOR: endpoint_from
#[derive(Debug)]
struct Endpoint {
    host: String,
    port: u16,
}

impl From<&str> for Endpoint {
    fn from(host: &str) -> Self {
        Endpoint { host: String::from(host), port: 80 }
    }
}
// ANCHOR_END: endpoint_from

// ANCHOR: connect_into
impl From<(&str, u16)> for Endpoint {
    fn from((host, port): (&str, u16)) -> Self {
        Endpoint { host: String::from(host), port }
    }
}

fn connect<T: Into<Endpoint>>(target: T) {
    let ep: Endpoint = target.into();
    println!("connecting to {}:{}", ep.host, ep.port);
}
// ANCHOR_END: connect_into

fn main() {
    connect("example.com");
    connect(("127.0.0.1", 8080));
    connect(Endpoint { host: String::from("localhost"), port: 3000 });
}
