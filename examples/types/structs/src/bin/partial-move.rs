// Expect: E0382
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

    let name = user.name;
    println!("moved name: {name}, remaining age: {}", user.age);

    // println!("cannot read moved field: {}", user.name);
    // println!("cannot use whole struct: {:?}", user);
}
