//! Monomorphization turns one generic function definition into distinct
//! compiled functions with different machine-code addresses and sizes.

use std::mem::size_of;

#[inline(never)]
fn swap_tuple<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

fn main() {
    let fn_u8: fn(u8, u8) -> (u8, u8) = swap_tuple::<u8>;
    let fn_u64: fn(u64, u64) -> (u64, u64) = swap_tuple::<u64>;
    let fn_string: fn(String, String) -> (String, String) = swap_tuple::<String>;

    let (x, y) = fn_u8(1, 2);
    let (m, n) = fn_u64(10, 20);
    let (s1, s2) = fn_string(String::from("a"), String::from("b"));

    println!("u8     ({}B) -> ({x}, {y}), code addr = {:#x}", size_of::<u8>(), fn_u8 as usize);
    println!("u64    ({}B) -> ({m}, {n}), code addr = {:#x}", size_of::<u64>(), fn_u64 as usize);
    println!("String ({}B) -> ({s1}, {s2}), code addr = {:#x}", size_of::<String>(), fn_string as usize);
    println!("distinct addresses? {}", (fn_u8 as usize) != (fn_u64 as usize));
}
