// Uncomment the marked `let bad = ...` block to trigger error[E0308].
fn main() {
    let score = 85;

    let grade = if score >= 90 {
        "A"
    } else if score >= 60 {
        "B"
    } else {
        "C"
    };
    // let bad = if score >= 60 { "pass" } else { 0 };

    let bonus = {
        let base = 10;
        base + 5
    };

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

    println!("grade = {grade}, bonus = {bonus}, code = {code}");
}
