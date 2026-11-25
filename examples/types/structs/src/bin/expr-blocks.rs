// Expect: E0308
// ANCHOR: min_max_fn
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}
// ANCHOR_END: min_max_fn

fn main() {
    // ANCHOR: block_expr
    let bonus = {
        let base = 10;
        base + 5
    };
    // ANCHOR_END: block_expr

    // ANCHOR: if_expr
    let score = 85;
    let grade = if score >= 90 {
        "A"
    } else if score >= 60 {
        "B"
    } else {
        "C"
    };
    let (low, high) = min_max(9, 2);
    // let bad = if score >= 60 { "pass" } else { 0 };
    // ANCHOR_END: if_expr

    // ANCHOR: loop_expr
    let mut count = 0;
    while count < 2 {
        count += 1;
    }

    let code = loop {
        count += 1;
        if count == 3 {
            break count * 100;
        }
    };
    // ANCHOR_END: loop_expr

    println!("bonus = {bonus}, grade = {grade}, range = ({low}, {high}), code = {code}");
}
