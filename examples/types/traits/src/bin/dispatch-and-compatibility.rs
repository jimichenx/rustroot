//! Comparing static dispatch (`<T: Summary>`, monomorphized per type) with
//! dynamic dispatch (`&dyn Summary`, single compiled function looking up `vtable`),
//! plus the three physical boundaries where a trait is not `dyn` compatible (`E0038`).

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

// 1. Static dispatch: compiler generates `notify_static::<Article>` and `notify_static::<Metric>`
fn notify_static<T: Summary>(item: &T) -> String {
    item.summarize()
}

// 2. Dynamic dispatch: only ONE `notify_dynamic` exists in the binary; looks up `vtable` at runtime
fn notify_dynamic(item: &dyn Summary) -> String {
    item.summarize()
}

// A trait with a generic method or returning `Self` cannot build a fixed-slot vtable:
#[allow(dead_code)]
trait NotDynCompatible {
    fn make_default() -> u32;          // no `&self` receiver -> no fat pointer to find vtable
    fn clone_self(&self) -> Self;      // returns `Self` by value -> caller doesn't know stack size
    fn tag<U>(&self, extra: U) -> u32; // generic `<U>` -> infinite monomorphized functions won't fit in vtable
}

fn main() {
    let post = Article { title: String::from("Rust") };
    let cpu = Metric { name: "cpu", value: 99 };

    println!("static:  {} | {}", notify_static(&post), notify_static(&cpu));
    println!("dynamic: {} | {}", notify_dynamic(&post), notify_dynamic(&cpu));

    // Uncommenting the line below triggers error[E0038]:
    // the trait `NotDynCompatible` is not dyn compatible
    // let _bad: &dyn NotDynCompatible;
}
