//! Moving one field out leaves the other fields usable.
//! Uncomment either commented `println!` line to trigger error[E0382].

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Ferris"),
        age: 7,
    };

    let name = user.name; // the String moves out to `name`
    println!("moved name: {name}, remaining age: {}", user.age);

    // println!("cannot read moved field: {}", user.name);
    // println!("cannot use whole struct: {:?}", user);
}
