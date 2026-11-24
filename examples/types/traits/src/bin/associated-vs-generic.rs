// Expect: E0119
// ANCHOR: generic_trait
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
// ANCHOR_END: generic_trait

// ANCHOR: assoc_type
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

// impl MyIterator for Counter {
//     type Item = String;
//     fn next_item(&mut self) -> Option<String> { None }
// }

fn inspect_first<C: MyIterator>(c: &mut C) -> Option<C::Item> {
    c.next_item()
}
// ANCHOR_END: assoc_type

fn main() {
    let p1 = Packet::my_from(64usize);
    let p2 = Packet::my_from("hello");
    println!("packets: {}, {}", p1.len, p2.len);

    let mut c = Counter { current: 0 };
    println!("first = {:?}", inspect_first(&mut c));
}
