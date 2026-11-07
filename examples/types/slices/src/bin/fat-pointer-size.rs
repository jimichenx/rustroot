use std::mem::size_of;

fn main() {
    let a: [i32; 4] = [10, 20, 30, 40];
    let r1: &[i32; 4] = &a;
    let r2: &[i32] = &a[1..3];

    println!("r1 (&[i32; 4]): {} bytes, first = {}", size_of::<&[i32; 4]>(), r1[0]);
    println!("r2 (&[i32]):    {} bytes, len   = {}", size_of::<&[i32]>(), r2.len());
}
