//! Three receivers, three ways of handing the value over.
//! Uncomment the last line to get error[E0382]: borrow of moved value: `c`.

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
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

fn main() {
    let mut c = Counter::new();
    c.bump();
    println!("{}", c.get());
    let total = c.finish(); // `c` moves into the method
    println!("{total}");
    // println!("{}", c.get());
}
