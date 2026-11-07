use std::mem::size_of;

fn main() {
    let a: [i32; 4] = [10, 20, 30, 40];
    let zeros = [0; 4];

    println!("first: {}, len: {}", a[0], a.len());
    println!("zeros: {:?}", zeros);
    println!("size of [i32; 4]: {} bytes", size_of::<[i32; 4]>());
}
