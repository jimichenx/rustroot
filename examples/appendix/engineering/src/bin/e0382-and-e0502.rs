//! Reproducing and fixing error[E0382] (use after move) and error[E0502] (borrow conflict).
//! Uncomment the marked lines to inspect the exact compiler spans.

// ANCHOR: full
fn log_tag(tag: &str) {
    println!("tag: {tag}");
}

fn main() {
    // 1. E0382: non-Copy String moves on value assignment or by-value argument
    let tag = String::from("release-v1");
    // let moved = tag;
    // println!("{tag}"); // error[E0382]: borrow of moved value: `tag`
    log_tag(&tag);
    println!("still owned: {tag}");

    // 2. E0502: shared borrow and mutable borrow cannot overlap in active span
    let mut scores = vec![10, 20, 30];
    let first = &scores[0];
    // scores.push(40); // error[E0502]: cannot borrow `scores` as mutable because it is also borrowed as immutable
    println!("first score: {first}"); // last read of `first` ends the shared borrow (NLL)
    scores.push(40);
    println!("updated scores: {scores:?}");
}
// ANCHOR_END: full
