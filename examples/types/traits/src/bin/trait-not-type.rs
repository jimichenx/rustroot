//! Defining a `Summary` trait and implementing it for two distinct structs.
//! Uncomment `fn bad_notify(item: Summary)` to see that a bare Trait is a
//! contract without a known stack size, not a concrete type (error[E0277] / E0782).

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

// A bare trait `Summary` is a contract, not a concrete value type:
// fn bad_notify(item: Summary) {} // error[E0277]: size cannot be known at compilation time

fn notify<T: Summary>(item: &T) {
    println!("[notify] {}", item.summarize());
}

fn main() {
    let post = Article {
        title: String::from("Rust Root"),
        author: String::from("Ferris"),
    };
    let cpu = Metric { name: "cpu_usage", value: 42 };

    notify(&post);
    notify(&cpu);
}
