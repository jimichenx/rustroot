// Expect: E0308
fn add(a: i32, b: i32) -> i32 {
    a + b
    // a + b;
}

fn swap(a: i32, b: bool) -> (bool, i32) {
    (b, a)
}

fn log_ready(tag: &str) {
    println!("[{tag}] ready");
}

fn main() {
    let sum: i32 = add(3, 4);
    let (ok, code) = swap(200, true);
    let unit_val: () = log_ready("init");

    println!("sum = {sum}, ok = {ok}, code = {code}, unit = {unit_val:?}");
}
