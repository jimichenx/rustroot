// Expect: E0308
fn main() {
    let score = 85;

    // ANCHOR: if_expr
    let grade = if score >= 90 {
        "A"
    } else if score >= 60 {
        "B"
    } else {
        "C"
    };
    // let bad = if score >= 60 { "pass" } else { 0 };
    // ANCHOR_END: if_expr

    // ANCHOR: block_expr
    let bonus = {
        let base = 10;
        base + 5
    };
    // ANCHOR_END: block_expr

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

    println!("grade = {grade}, bonus = {bonus}, code = {code}");
}
