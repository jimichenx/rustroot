// Expect: E0502, E0507
fn main() {
    // ANCHOR: basics
    let mut scores = vec![10, 20];
    scores.push(30);

    let empty: Vec<i32> = Vec::new();
    println!("first: {}, len: {}, empty len: {}", scores[0], scores.len(), empty.len());
    // ANCHOR_END: basics

    // ANCHOR: push_while_borrowed
    let mut names = vec![String::from("Alice"), String::from("Bob")];
    let leader = &names[0];
    // names.push(String::from("Carol"));
    println!("leader: {leader}");
    // ANCHOR_END: push_while_borrowed

    // ANCHOR: move_out_of_index
    // let first = names[0];
    println!("get(1): {:?}, get(99): {:?}", names.get(1), names.get(99));

    let popped = names.pop();
    println!("popped: {:?}, remaining: {:?}", popped, names);
    // ANCHOR_END: move_out_of_index
}
