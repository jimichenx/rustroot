fn main() {
    // 1. Two `&i32` references pointing to different scopes have distinct concrete types:
    let outer_val = 10;
    let r_outer: &i32 = &outer_val;

    {
        let inner_val = 20;
        let r_inner: &i32 = &inner_val;
        println!("r_outer = {r_outer}, r_inner = {r_inner}");

        // Uncomment to trigger E0597 (`inner_val` does not live long enough):
        // r_outer = &inner_val;
    }

    // 2. Non-Lexical Lifetimes (NLL): a borrow ends at its last use:
    let mut scores = vec![10, 20, 30];
    let first_view: &i32 = &scores[0];
    println!("first score = {first_view}");

    scores.push(40);
    println!("updated scores = {scores:?}");
}
