//! The static answer and the dynamic answer on the same data shape.
//! `[i32; 5]` is a plain value: assignment copies all 20 bytes,
//! and both bindings stay usable (Copy).
//! `Vec<i32>` owns its elements: the handle alone is copied and the
//! release duty travels with it, so the old binding dies (Move).

fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = a; // copies all five integers
    println!("{} {}", b[0], a[0]); // both bindings still work

    let v = vec![1, 2, 3, 4, 5];
    let w = v; // moves: the three-word handle goes to `w`
    println!("{}", w[0]);
    // println!("{}", v[0]); // error[E0382]: borrow of moved value: `v`
}
