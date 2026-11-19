// Expect: E0106
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
