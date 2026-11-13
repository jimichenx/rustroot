// ANCHOR: full
use std::borrow::Borrow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ANCHOR: path_and_slice
pub fn has_toml_ext<P: AsRef<Path>>(path: P) -> bool {
    // Thin generic shim converts to `&Path`, delegating to a single non-generic function
    has_toml_ext_inner(path.as_ref())
}

fn has_toml_ext_inner(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "toml")
}

pub fn join_hosts<S: AsRef<str>>(hosts: &[S]) -> String {
    let mut out = String::new();
    for (idx, host) in hosts.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str(host.as_ref());
    }
    out
}
// ANCHOR_END: path_and_slice

// ANCHOR: borrow_lookup
struct RouteEntry<K> {
    key: K,
    port: u16,
}

impl<K> RouteEntry<K> {
    fn match_port<Q: ?Sized>(&self, query: &Q) -> Option<u16>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        if self.key.borrow() == query {
            Some(self.port)
        } else {
            None
        }
    }
}
// ANCHOR_END: borrow_lookup

// ANCHOR: main_usage
fn main() {
    let owned_path = PathBuf::from("config/service.toml");
    // Pass `&owned_path` so `P = &PathBuf`, preserving ownership of `owned_path`
    println!(
        "toml? literal={}, borrowed={}",
        has_toml_ext("Cargo.toml"),
        has_toml_ext(&owned_path),
    );
    println!("owned_path still valid: {}", owned_path.display());

    let static_hosts: &[&str] = &["db-primary", "db-replica"];
    let dynamic_hosts: Vec<String> = vec![String::from("cache-01"), String::from("cache-02")];
    println!("static  = {}", join_hosts(static_hosts));
    println!("dynamic = {}", join_hosts(&dynamic_hosts));

    let route = RouteEntry {
        key: String::from("gateway"),
        port: 8443,
    };
    let mut ports = HashMap::new();
    ports.insert(String::from("gateway"), 8443u16);
    println!(
        "borrow lookup: entry={:?}, map={:?}",
        route.match_port("gateway"),
        ports.get("gateway"),
    );
}
// ANCHOR_END: main_usage
// ANCHOR_END: full
