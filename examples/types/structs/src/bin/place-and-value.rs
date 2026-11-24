// Expect: E0070, E0508, E0507
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // ANCHOR: place_chain
    let mut people = [
        Person { name: String::from("Alice"), age: 28 },
        Person { name: String::from("Bob"), age: 31 },
    ];

    people[0].name = String::from("Alicia");
    let name_slot: &mut String = &mut people[0].name;
    *name_slot = String::from("Al");
    // people.len() = 1;
    println!("updated: {} ({})", people[0].name, people[0].age);
    // ANCHOR_END: place_chain

    // ANCHOR: move_from_places
    let single = Person { name: String::from("Carol"), age: 25 };
    let moved_field = single.name;
    let r: &String = &moved_field;

    // let _from_index = people[1].name;
    // let _from_ref = *r;
    println!("moved: {moved_field}, rest age: {}, borrowed: {r}", single.age);
    // ANCHOR_END: move_from_places
}
