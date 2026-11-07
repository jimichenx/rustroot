//! A paid tag is padded to the payload's alignment, not to the variant count.

use std::mem::size_of;

#[allow(dead_code)]
enum TagU8 {
    A,
    B,
    P(u8),
}

#[allow(dead_code)]
enum TagU64 {
    A,
    B,
    P(u64),
}

fn main() {
    println!("TagU8   {}", size_of::<TagU8>());
    println!("TagU64  {}", size_of::<TagU64>());
}
