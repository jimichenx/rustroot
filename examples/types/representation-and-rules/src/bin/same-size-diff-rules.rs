// Uncomment the marked line to trigger error[E0382].
fn main() {
    let x = 7;
    let r1: &i32 = &x;
    let r2 = r1;
    println!("{r1} {r2}");

    let mut y = 7;
    let m1: &mut i32 = &mut y;
    let m2 = m1;
    // println!("{m1}");
    println!("{m2}");
}
