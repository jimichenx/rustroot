struct Counter {
    count: u32,
}

impl Counter {
    fn get(&self) -> u32 {
        self.count
    }
}

fn main() {
    // ANCHOR: autoref
    let mut v = vec![1, 2, 3];
    v.push(4);
    (&mut v).push(5);
    Vec::push(&mut v, 6);
    // ANCHOR_END: autoref

    // ANCHOR: autoderef
    let c = Counter { count: 7 };
    let r: &Counter = &c;
    let rr: &&Counter = &r;
    println!("field via ref: {} == {}", r.count, (*r).count);
    println!(
        "method via &&Counter: {} == {} == {}",
        rr.get(),
        (*rr).get(),
        Counter::get(*rr)
    );
    // ANCHOR_END: autoderef
    println!("vec: {v:?}");
}
