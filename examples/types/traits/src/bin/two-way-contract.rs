//! A trait bound `<T: Summary>` is a two-way compile-time contract:
//! 1. Inside `notify`, only methods from `Summary` are allowed (uncomment `item.value` -> E0609).
//! 2. At call sites, types without `impl Summary` are rejected (uncomment `notify(&raw)` -> E0277).

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

fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
    // 1. Inside the body, accessing fields/methods outside `Summary` is forbidden:
    // println!("{}", item.value); // error[E0609]: no field `value` on type `&T`
}

fn main() {
    let m = Metric { name: "latency_ms", value: 12 };
    notify(&m);

    let raw = Unregistered { id: 1 };
    println!("unregistered id = {}", raw.id);
    // 2. At the call site, passing a type that hasn't implemented `Summary` fails:
    // notify(&raw); // error[E0277]: the trait bound `Unregistered: Summary` is not satisfied
}
