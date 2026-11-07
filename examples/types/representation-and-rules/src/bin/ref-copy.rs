//! `&i32` and `&mut i32` are both one machine word with no heap behind them.
//! Assigning `&i32` copies it; assigning `&mut i32` moves it.
//! Uncomment the last line to get error[E0382]: use of moved value: `a`.

fn main() {
    let x = 1;
    let r = &x;
    let s = r;
    println!("{r} {s}");

    let mut y = 1;
    let a: &mut i32 = &mut y;
    let b = a;
    *b += 1;
    // *a += 1;
}
