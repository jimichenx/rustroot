// Expect: E0382
use std::mem::size_of_val;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let min_val = 2;

    // ANCHOR: lazy_pipeline
    let pipeline = nums
        .iter()
        .filter(|&&x| {
            println!("  [filter] checking {x}");
            x > min_val
        })
        .map(|&x| {
            println!("  [map] squaring {x}");
            x * x
        });

    println!(
        "pipeline built (stack size = {}B), nothing printed yet",
        size_of_val(&pipeline)
    );

    let total: i32 = pipeline.sum();
    println!("total = {total}");
    // ANCHOR_END: lazy_pipeline

    // ANCHOR: collect_and_combinator
    let evens: Vec<i32> = nums.iter().copied().filter(|&x| x % 2 == 0).collect();
    println!("evens = {evens:?}");

    let raw_port = Some("8080");
    let port: u16 = raw_port
        .map(|s| s.trim())
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or_else(|| 3000);
    println!("port = {port}");
    // ANCHOR_END: collect_and_combinator

    // println!("run again? {}", pipeline.sum::<i32>());
}
