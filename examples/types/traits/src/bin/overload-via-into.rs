//! Replacing traditional function overloading with a single generic function
//! constrained by `Into<Endpoint>`.

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

impl From<(&str, u16)> for Endpoint {
    fn from((host, port): (&str, u16)) -> Self {
        Endpoint { host: String::from(host), port }
    }
}

// One function accepts any type `T` that can be converted `Into<Endpoint>`:
fn connect<T: Into<Endpoint>>(target: T) {
    let ep: Endpoint = target.into();
    println!("connecting to {}:{}", ep.host, ep.port);
}

fn main() {
    connect("example.com");
    connect(("127.0.0.1", 8080));
    connect(Endpoint { host: String::from("localhost"), port: 3000 });
}
