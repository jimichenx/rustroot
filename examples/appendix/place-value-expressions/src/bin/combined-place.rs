struct Person {
    name: String,
}

fn main() {
    let mut people = [
        Person { name: String::from("Alice") },
        Person { name: String::from("Bob") },
    ];

    people[0].name = String::from("Alicia");

    let first_name = &mut people[0].name;
    *first_name = String::from("Al");

    println!("{}", people[0].name);
}
