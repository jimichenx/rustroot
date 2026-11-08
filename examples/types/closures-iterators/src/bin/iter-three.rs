//! Lazy iterator adapters (`filter`, `map`) vs driving consumers (`sum`, `collect`),
//! plus `Option` / `Result` combinators (`map`, `and_then`, `unwrap_or_else`).
//! Uncomment the final `println!` to observe `error[E0382]`.

use std::mem::size_of_val;

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let min_val = 2;

    // 1. Building an adapter chain is lazy: it only constructs a nested stack struct
    //    `Map<Filter<slice::Iter<i32>, Closure1>, Closure2>` without running the closures.
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

    // 2. Calling a consumer (`sum`) drives `.next()` until `None`:
    let total: i32 = pipeline.sum();
    println!("total = {total}");

    // 3. `collect` uses `FromIterator` to build a target collection:
    let evens: Vec<i32> = nums.iter().copied().filter(|&x| x % 2 == 0).collect();
    println!("evens = {evens:?}");

    // 4. `Option` / `Result` combinators share the exact same closure design:
    let raw_port = Some("8080");
    let port: u16 = raw_port
        .map(|s| s.trim())
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or_else(|| 3000);
    println!("port = {port}");

    // Uncomment to observe error[E0382]: `sum(self)` consumed `pipeline`
    // println!("run again? {}", pipeline.sum::<i32>());
}
