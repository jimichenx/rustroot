use std::mem::size_of;

fn print_greeting(text: &str) {
    println!("greeting ({} bytes): {text}", text.len());
}

fn main() {
    let owned: String = String::from("hello world");
    let borrowed_full: &String = &owned;
    let slice_part: &str = &owned[0..5];
    let literal: &str = "hello rust";

    println!("String:  {} bytes", size_of::<String>());
    println!("&String: {} bytes", size_of::<&String>());
    println!("&str:    {} bytes", size_of::<&str>());

    print_greeting(borrowed_full);
    print_greeting(slice_part);
    print_greeting(literal);
}
