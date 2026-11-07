// Uncomment either marked line to trigger error[E0508] or error[E0382].
fn main() {
    let a = [1, 2, 3, 4];
    let b = a;
    println!("int array copy: {} {}", a[0], b[0]);

    let names = [String::from("Alice"), String::from("Bob")];
    // let first = names[0];
    let first_ref: &String = &names[0];
    println!("borrowed element: {first_ref}");

    let moved_names = names;
    // println!("{}", names[0]);
    println!("moved array: {}", moved_names[0]);
}
