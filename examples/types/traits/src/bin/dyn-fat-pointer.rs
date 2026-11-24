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

// ANCHOR: solve_boundaries
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

    let dashboard: Vec<&dyn Summary> = vec![&post, &cpu];
    for widget in dashboard {
        println!("dashboard item -> {}", widget.summarize());
    }

    let dynamic_choice = choose_item(false);
    println!("chosen at runtime -> {}", dynamic_choice.summarize());
}
// ANCHOR_END: solve_boundaries
