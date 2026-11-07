// Uncomment either marked line to trigger error[E0502] or error[E0507].
#[allow(unused_mut)]
fn main() {
    let mut scores = vec![10, 20, 30];
    let first_ref = &scores[0];
    // scores.push(40);
    println!("first score: {first_ref}");
    println!("safe get(1): {:?}, safe get(99): {:?}", scores.get(1), scores.get(99));

    let mut names = vec![String::from("Alice"), String::from("Bob")];
    // let stolen = names[0];
    let popped = names.pop();
    println!("popped: {:?}, remaining: {:?}", popped, names);
}
