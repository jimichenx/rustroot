use std::mem::size_of;

trait Summary {
    fn summarize(&self) -> String;
}

struct Metric {
    name: &'static str,
    value: u64,
}

impl Summary for Metric {
    fn summarize(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

fn main() {
    let vec_nums: Vec<i32> = vec![10, 20, 30, 40];
    let boxed_slice: Box<[i32]> = vec_nums.into_boxed_slice();
    let boxed_dyn: Box<dyn Summary> = Box::new(Metric {
        name: "latency_ms",
        value: 12,
    });

    println!(
        "Vec<i32> = {}B, Box<[i32]> = {}B, Box<dyn Summary> = {}B",
        size_of::<Vec<i32>>(),
        size_of::<Box<[i32]>>(),
        size_of::<Box<dyn Summary>>()
    );
    println!("boxed_slice sum = {}", boxed_slice.iter().sum::<i32>());
    println!("boxed_dyn -> {}", boxed_dyn.summarize());
}
