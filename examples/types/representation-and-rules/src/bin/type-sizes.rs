use std::mem::size_of;

fn main() {
    println!("i32:       {} bytes", size_of::<i32>());
    println!("&i32:      {} bytes", size_of::<&i32>());
    println!("&mut i32:  {} bytes", size_of::<&mut i32>());
    println!("String:    {} bytes", size_of::<String>());
    println!("Vec<u8>:   {} bytes", size_of::<Vec<u8>>());
}
