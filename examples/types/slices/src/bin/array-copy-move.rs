// Expect: E0508, E0382
fn main() {
    // ANCHOR: copy_vs_move
    let a = [1, 2, 3, 4];
    let b = a;
    println!("int array copy: {} {}", a[0], b[0]);

    let names = [String::from("Alice"), String::from("Bob")];
    // ANCHOR_END: copy_vs_move
    // ANCHOR: index_move
    // let first = names[0];
    let first_ref: &String = &names[0];
    println!("borrowed element: {first_ref}");
    // ANCHOR_END: index_move

    // ANCHOR: whole_move
    let moved_names = names;
    // println!("{}", names[0]);
    println!("moved array: {}", moved_names[0]);
    // ANCHOR_END: whole_move
}
