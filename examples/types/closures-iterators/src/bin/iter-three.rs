//! Three ways to take elements: shared borrow, mutable borrow, ownership.
//! Uncomment the last line to get error[E0382]: borrow of moved value: `names`.

fn main() {
    let mut names = vec![String::from("ferris"), String::from("crab")];

    let lengths: Vec<usize> = names.iter().map(|n| n.len()).collect();
    println!("{lengths:?}");

    for n in names.iter_mut() {
        n.push('!');
    }

    let long: Vec<String> = names.into_iter().filter(|n| n.len() > 5).collect();
    println!("{long:?}");
    // println!("{names:?}");
}
