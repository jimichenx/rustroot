use std::mem::size_of;

fn log_message(msg: &str) {
    println!("{msg}");
}

fn main() {
    let pair: (i32, bool) = (42, true);
    let (num, ok) = pair;
    let first = pair.0;
    let result: () = log_message("ready");
    println!("{num} {ok} {first} {:?} size_of_()={}", result, size_of::<()>());
}
