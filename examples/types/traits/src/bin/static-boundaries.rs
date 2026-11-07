//! `impl Trait` in argument and return positions, plus the two boundaries
//! where compile-time static monomorphization hits a wall:
//! 1. Runtime branches returning different concrete types (`error[E0308]`)
//! 2. Heterogeneous collections storing different concrete types (`error[E0308]`)

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

// 1. `impl Summary` in argument position (sugar for `<T: Summary>`):
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 2. `impl Summary` in return position (must resolve to ONE concrete type):
fn make_default_metric() -> impl Summary {
    Metric { name: "cpu", value: 10 }
}

// Boundary 1: Returning different concrete types across runtime branches fails!
// fn choose_item(is_article: bool) -> impl Summary {
//     if is_article {
//         Article { title: String::from("News") }
//     } else {
//         Metric { name: "cpu", value: 10 } // error[E0308]: `if` and `else` have incompatible types
//     }
// }

fn main() {
    let a = Article { title: String::from("Rust 1.0") };
    let m = make_default_metric();
    notify(&a);
    notify(&m);

    // Boundary 2: A generic `Vec<T>` cannot hold both `Article` and `Metric` directly!
    // let _items = vec![
    //     Article { title: String::from("News") },
    //     Metric { name: "cpu", value: 10 }, // error[E0308]: mismatched types
    // ];
}
