// Uncomment the marked line to trigger error[E0384].
fn main() {
    let mut x = 1;
    let mut y = 10;

    let r: &mut i32 = &mut x;
    *r += 1;
    println!("{x}");

    // r = &mut y;
    println!("{y}");
}
