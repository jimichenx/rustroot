// Expect: E0369
fn keep_second<T>(_first: T, second: T) -> T {
    second
}

// fn add_values<T>(a: T, b: T) -> T {
//     a + b
// }

fn main() {
    let n = keep_second(10, 20);
    let s = keep_second(String::from("old"), String::from("new"));
    println!("n = {n}, s = {s}");
}
