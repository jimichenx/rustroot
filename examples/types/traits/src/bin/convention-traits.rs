//! Ecosystem convention traits: `Default`, `From`/`Into`, `TryFrom`, and `AsRef`.

#[derive(Debug, Default, PartialEq)]
struct Port(u16);

// 1. Infallible conversion (`From` automatically unlocks `Into`):
impl From<u16> for Port {
    fn from(raw: u16) -> Self {
        Port(raw)
    }
}

// 2. Fallible conversion (`TryFrom` returns `Result`):
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

// 3. `AsRef<str>` allows borrowing both `&str` and `String` with zero allocation:
fn print_endpoint<S: AsRef<str>>(host: S, port: Port) {
    println!("{}:{}", host.as_ref(), port.0);
}

fn main() {
    let default_port = Port::default();
    let http: Port = 80u16.into(); // unlocked by `From<u16>`
    let checked = Port::try_from(8080u32).unwrap();

    print_endpoint("localhost", default_port);
    print_endpoint(String::from("127.0.0.1"), http);
    println!("checked = {checked:?}, invalid = {:?}", Port::try_from(99999u32));
}
