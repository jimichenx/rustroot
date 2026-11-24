// Expect: E0782
// ANCHOR: define_and_impl
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

struct Metric {
    name: &'static str,
    value: u64,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

impl Summary for Metric {
    fn summarize(&self) -> String {
        format!("{} = {}", self.name, self.value)
    }
}
// ANCHOR_END: define_and_impl

// ANCHOR: notify_bound
// fn bad_notify(item: Summary) {}

fn notify<T: Summary>(item: &T) {
    println!("[notify] {}", item.summarize());
}
// ANCHOR_END: notify_bound

fn main() {
    let post = Article {
        title: String::from("Rust Root"),
        author: String::from("Ferris"),
    };
    let cpu = Metric { name: "cpu_usage", value: 42 };

    notify(&post);
    notify(&cpu);
}
