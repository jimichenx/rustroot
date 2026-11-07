//! The index answer: a plain `usize` compiles and runs,
//! but the number has no recorded connection to `s`.
//! Uncomment the last line to get a runtime panic, not a compile error.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let end = first_word(&s);

    s.clear();

    println!("end = {end}, s.len() = {}", s.len());
    // println!("{}", &s[..end]); // panics: byte index out of bounds
}
