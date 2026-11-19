// Expect: E0382
// ANCHOR: assoc_fn
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}
// ANCHOR_END: assoc_fn

// ANCHOR: three_receivers
impl Counter {
    fn get(&self) -> u32 {
        self.count
    }

    fn bump(&mut self) {
        self.count += 1;
    }

    fn finish(self) -> u32 {
        self.count
    }
}
// ANCHOR_END: three_receivers

fn main() {
    let mut c = Counter::new();
    c.bump();
    println!("current: {}", c.get());
    let total = c.finish();
    println!("finished: {total}");
    // println!("after finish: {}", c.get());
}
