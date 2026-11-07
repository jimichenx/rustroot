//! Values made only of stack bytes remain usable after assignment.

fn main() {
    let a = 1;
    let b = a;
    println!("{a}");
    println!("{b}");
}
