//! A `for` loop and its manual expansion produce the same output.

fn main() {
    let v = vec![10, 20, 30];

    for x in &v {
        println!("for:    {x}");
    }

    let mut it = (&v).into_iter(); // the one conversion
    loop {
        match it.next() { // one call per round
            Some(x) => println!("manual: {x}"),
            None => break,
        }
    }
}
