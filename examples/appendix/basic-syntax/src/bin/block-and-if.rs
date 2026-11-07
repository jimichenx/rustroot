fn main() {
    let score = 85;
    let level = if score >= 60 { "pass" } else { "fail" };

    let mut n = 1;
    let doubled = loop {
        n = n + 1;
        if n == 4 {
            break n * 2;
        }
    };
    println!("{level} {doubled}");
}
