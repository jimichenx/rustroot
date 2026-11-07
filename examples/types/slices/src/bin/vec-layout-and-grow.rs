use std::mem::size_of;

fn main() {
    let mut nums: Vec<i32> = Vec::with_capacity(2);
    println!(
        "empty: len = {}, cap = {}, stack size = {} bytes",
        nums.len(),
        nums.capacity(),
        size_of::<Vec<i32>>()
    );

    nums.push(10);
    nums.push(20);
    while nums.len() < nums.capacity() {
        nums.push(0);
    }
    let ptr_before = nums.as_ptr();
    println!("full:  len = {}, cap = {}, heap ptr = {:p}", nums.len(), nums.capacity(), ptr_before);

    // The buffer is full, so this push must grow its capacity.
    nums.push(30);
    let ptr_after = nums.as_ptr();
    println!("grown: len = {}, cap = {}, heap ptr = {:p}", nums.len(), nums.capacity(), ptr_after);
}
