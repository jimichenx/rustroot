//! A binding may be declared first and assigned later,
//! as long as every path assigns it before it is read.
//! Comment out `value = 2;` to get error[E0381]: used binding `value`
//! is possibly-uninitialized.

fn main() {
    let ready = std::env::args().count() > 1;
    let value: i32;

    if ready {
        value = 1;
    } else {
        value = 2;
    }

    println!("{value}");
}
