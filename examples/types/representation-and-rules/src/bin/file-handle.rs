//! A `File` holds only the number the operating system gave it. No heap.
//! It still moves: the file must be closed exactly once.
//! Run from the `examples` directory so that `Cargo.toml` exists.
//! Uncomment the last line to get error[E0382]: use of moved value: `f`.

use std::fs::File;
use std::mem::size_of;

fn main() {
    println!("File: {} bytes", size_of::<File>());

    let f = File::open("Cargo.toml").expect("run from the examples directory");
    let g = f;
    println!("{g:?}");
    // println!("{f:?}");
}
