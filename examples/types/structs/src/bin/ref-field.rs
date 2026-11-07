//! A field that holds a reference needs a lifetime parameter on the type.
//! Uncomment the struct to get error[E0106]: missing lifetime specifier.
//! Lesson 2.11 writes the parameter. Here the field owns its text.

// struct BorrowedExcerpt {
//     part: &str,
// }

struct Excerpt {
    part: String,
}

fn main() {
    let e = Excerpt { part: String::from("Call me Ishmael.") };
    println!("{}", e.part);
}
