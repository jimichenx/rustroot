// Expect: E0308
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

// ANCHOR: impl_trait_and_branch
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn make_default_metric() -> impl Summary {
    Metric { name: "cpu", value: 10 }
}

// fn choose_item(is_article: bool) -> impl Summary {
//     if is_article {
//         Article { title: String::from("News") }
//     } else {
//         Metric { name: "cpu", value: 10 }
//     }
// }
// ANCHOR_END: impl_trait_and_branch

fn main() {
    let a = Article { title: String::from("Rust 1.0") };
    let m = make_default_metric();
    notify(&a);
    notify(&m);

    // ANCHOR: hetero_vec
    // let _items = vec![
    //     Article { title: String::from("News") },
    //     Metric { name: "cpu", value: 10 },
    // ];
    // ANCHOR_END: hetero_vec
}
