//! Desugaring a `for` loop and comparing the three traversal modes:
//! shared borrow (`&names`), exclusive borrow (`&mut names`), and ownership (`names`).
//! Uncomment the final `println!` to observe `error[E0382]`.

fn main() {
    let sample = vec![10, 20];

    // 1. Standard `for` loop syntax
    for n in &sample {
        println!("for:    {n}");
    }

    // 2. What the compiler mechanically expands `for` into:
    {
        let mut iter = IntoIterator::into_iter(&sample);
        while let Some(n) = iter.next() {
            println!("manual: {n}");
        }
    }

    // 3. The three traversal modes on the same collection:
    let mut names = vec![String::from("Rust"), String::from("Root")];

    // Mode A: `for s in &names` -> `Self` is `&Vec<String>`, `Item` is `&String`
    for s in &names {
        let _: &String = s;
        println!("read: {s} (len = {})", s.len());
    }

    // Mode B: `for s in &mut names` -> `Self` is `&mut Vec<String>`, `Item` is `&mut String`
    for s in &mut names {
        let _: &mut String = s;
        s.push('!');
    }
    println!("after mut loop: {names:?}");

    // Mode C: `for s in names` -> `Self` is `Vec<String>`, `Item` is `String` (consumes `names`)
    for s in names {
        let _: String = s;
        println!("consumed: {s}");
    }

    // Uncomment to observe error[E0382]: borrow of moved value: `names`
    // println!("still here? {names:?}");
}
