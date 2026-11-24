// Expect: E0038
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
}

struct Metric {
    name: &'static str,
    value: u64,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

impl Summary for Metric {
    fn summarize(&self) -> String {
        format!("Metric: {}={}", self.name, self.value)
    }
}

// ANCHOR: dispatch_pair
fn notify_static<T: Summary>(item: &T) -> String {
    item.summarize()
}

fn notify_dynamic(item: &dyn Summary) -> String {
    item.summarize()
}
// ANCHOR_END: dispatch_pair

// ANCHOR: not_dyn_compatible
#[allow(dead_code)]
trait NotDynCompatible {
    fn make_default() -> u32;
    fn clone_self(&self) -> Self;
    fn tag<U>(&self, extra: U) -> u32;
}
// ANCHOR_END: not_dyn_compatible

fn main() {
    let post = Article { title: String::from("Rust") };
    let cpu = Metric { name: "cpu", value: 99 };

    println!("static:  {} | {}", notify_static(&post), notify_static(&cpu));
    println!("dynamic: {} | {}", notify_dynamic(&post), notify_dynamic(&cpu));

    // let _bad: &dyn NotDynCompatible;
}
