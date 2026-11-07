use std::mem::size_of_val;

fn main() {
    let base = 10;
    let c1 = |x: i32| x + 1;
    let c2 = |x: i32| x + base;

    println!("closure (no capture): {} bytes", size_of_val(&c1));
    println!("closure (capture &i32): {} bytes", size_of_val(&c2));
}
