// Expect: E0382
fn main() {
    let sample = vec![10, 20];

    for n in &sample {
        println!("for:    {n}");
    }

    {
        let mut iter = IntoIterator::into_iter(&sample);
        while let Some(n) = iter.next() {
            println!("manual: {n}");
        }
    }

    // ANCHOR: three_modes
    let mut names = vec![String::from("Rust"), String::from("Root")];

    for s in &names {
        let _: &String = s;
        println!("read: {s} (len = {})", s.len());
    }

    for s in &mut names {
        let _: &mut String = s;
        s.push('!');
    }
    println!("after mut loop: {names:?}");

    for s in names {
        let _: String = s;
        println!("consumed: {s}");
    }

    // println!("still here? {names:?}");
    // ANCHOR_END: three_modes
}
