//! Binding by value in a pattern moves the data out of the enum.
//! Uncomment the last line to get error[E0382]: borrow of partially moved value: `maybe`.

fn main() {
    let maybe = Some(String::from("Ferris"));

    match maybe {
        Some(name) => println!("{name}"), // the String moves into `name`
        None => println!("nobody"),
    }
    // println!("{maybe:?}");
}
