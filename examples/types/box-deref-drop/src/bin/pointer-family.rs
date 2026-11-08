use std::mem::size_of;

fn main() {
    let val: i32 = 42;

    // 1. Reference (&T): borrows `val`, no ownership, compile-time borrow check
    let r: &i32 = &val;

    // 2. Raw pointer (*const T): pure address, no compile-time guarantees
    let raw: *const i32 = &val as *const i32;

    // 3. Smart pointer (Box<T>): owns heap data, Deref + Drop
    let boxed: Box<i32> = Box::new(42);

    println!(
        "stack sizes: &i32 = {}B, *const i32 = {}B, Box<i32> = {}B",
        size_of::<&i32>(),
        size_of::<*const i32>(),
        size_of::<Box<i32>>()
    );
    println!("values via Deref: *r = {}, *boxed = {}", *r, *boxed);

    // Dereferencing a raw pointer requires an explicit `unsafe` block
    unsafe {
        println!("value via raw pointer: *raw = {}", *raw);
    }
}
