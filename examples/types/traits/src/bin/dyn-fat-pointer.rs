//! Measuring `&Article` (8-byte thin pointer) vs `&dyn Summary` (16-byte fat pointer),
//! and using `&dyn Summary` / `Box<dyn Summary>` to solve heterogeneous collections
//! and runtime-branch return types.

use std::mem::size_of;

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

// Solving Boundary 2 from 8.1: returning different concrete types across branches!
fn choose_item(is_article: bool) -> Box<dyn Summary> {
    if is_article {
        Box::new(Article { title: String::from("Breaking") })
    } else {
        Box::new(Metric { name: "mem_mb", value: 512 })
    }
}

fn main() {
    let post = Article { title: String::from("News") };
    let cpu = Metric { name: "cpu", value: 42 };

    let thin_ref: &Article = &post;
    let fat_ref: &dyn Summary = &post;
    println!("size of &Article      = {} bytes", size_of::<&Article>());
    println!("size of &dyn Summary  = {} bytes", size_of::<&dyn Summary>());
    println!("thin = {}, fat = {}", thin_ref.summarize(), fat_ref.summarize());

    // Solving Boundary 1 from 8.1: heterogeneous `Vec<&dyn Summary>`!
    let dashboard: Vec<&dyn Summary> = vec![&post, &cpu];
    for widget in dashboard {
        println!("dashboard item -> {}", widget.summarize());
    }

    let dynamic_choice = choose_item(false);
    println!("chosen at runtime -> {}", dynamic_choice.summarize());
}
