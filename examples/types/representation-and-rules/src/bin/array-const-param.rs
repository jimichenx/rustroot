// Uncomment the marked line to trigger error[E0308].
fn main() {
    let a: [i32; 3] = [1, 2, 3];
    let b: [i32; 4] = [1, 2, 3, 4];
    // let c: [i32; 3] = b;
    println!("{} {}", a[0], b[0]);
}
