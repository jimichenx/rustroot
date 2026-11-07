//! Comparing a generic trait (`MyFrom<T>`, one-to-many) with an associated
//! type (`MyIterator::Item`, one-to-one).
//! Uncomment the second `impl MyIterator for Counter` to get error[E0119]:
//! conflicting implementations of trait `MyIterator` for type `Counter`.

// 1. Generic Trait: one type (`Packet`) can implement `MyFrom<T>` for many `T`s.
trait MyFrom<T> {
    fn my_from(src: T) -> Self;
}

struct Packet {
    len: usize,
}

impl MyFrom<usize> for Packet {
    fn my_from(len: usize) -> Self {
        Packet { len }
    }
}

impl MyFrom<&str> for Packet {
    fn my_from(text: &str) -> Self {
        Packet { len: text.len() }
    }
}

// 2. Associated Type: one type (`Counter`) can only implement `MyIterator` once,
// locking `Item` to a single concrete type (`u32`).
trait MyIterator {
    type Item;
    fn next_item(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: u32,
}

impl MyIterator for Counter {
    type Item = u32;

    fn next_item(&mut self) -> Option<u32> {
        self.current += 1;
        Some(self.current)
    }
}

// Forbidden: `Counter` cannot implement `MyIterator` a second time (error[E0119]).
// impl MyIterator for Counter {
//     type Item = String;
//     fn next_item(&mut self) -> Option<String> { None }
// }

fn main() {
    let p1 = Packet::my_from(64usize);
    let p2 = Packet::my_from("hello");
    println!("p1.len = {}, p2.len = {}", p1.len, p2.len);

    let mut c = Counter { current: 0 };
    // No type annotation needed: `Counter::Item` is uniquely locked to `u32`.
    let first = c.next_item().unwrap();
    println!("first counter item = {first}");
}
