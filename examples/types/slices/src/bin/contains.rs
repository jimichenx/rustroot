//! A function that only reads asks for views, not ownership.
//! Owners, literals, and views into other data all fit `&str`.
//! The algorithm itself runs on views: the window
//! `&haystack[start..start + n]` is a slice too.

fn contains(haystack: &str, needle: &str) -> bool {
    let n = needle.len();
    if n > haystack.len() {
        return false;
    }
    for start in 0..=haystack.len() - n {
        if &haystack[start..start + n] == needle {
            return true;
        }
    }
    false
}

fn main() {
    let s = String::from("hello world");

    println!("{}", contains(&s, "wor")); // owner reference + literal
    println!("{}", contains("hello", "xyz")); // literal + literal

    let t = String::from("world");
    println!("{}", contains(&s[6..], &t[..2])); // two views into other data
}
