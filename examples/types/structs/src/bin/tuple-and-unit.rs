use std::mem::size_of;

fn main() {
    // ANCHOR: tuple_basics
    let record: (i32, bool) = (200, true);
    let (status, ok) = record;
    println!("status = {status}, ok = {ok}, first = {}", record.0);
    // ANCHOR_END: tuple_basics

    // ANCHOR: unit_basics
    let empty: () = ();
    println!("unit = {empty:?}, size of () = {}B", size_of::<()>());
    // ANCHOR_END: unit_basics
}
