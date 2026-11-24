// Expect: E0609, E0277
trait Summary {
    fn summarize(&self) -> String;
}

struct Metric {
    name: &'static str,
    value: u64,
}

impl Summary for Metric {
    fn summarize(&self) -> String {
        format!("{} = {}", self.name, self.value)
    }
}

struct Unregistered {
    id: u32,
}

// ANCHOR: two_way
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
    // println!("{}", item.value);
}

fn main() {
    let m = Metric { name: "latency_ms", value: 12 };
    notify(&m);

    let raw = Unregistered { id: 1 };
    println!("unregistered id = {}", raw.id);
    // notify(&raw);
}
// ANCHOR_END: two_way
