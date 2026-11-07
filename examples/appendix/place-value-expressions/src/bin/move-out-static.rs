//! Uncomment the marked line and compile to get error[E0507].

static TEXT: String = String::new();

fn main() {
    // let s = TEXT; // error[E0507]: cannot move out of static item
    println!("{TEXT}");
}
