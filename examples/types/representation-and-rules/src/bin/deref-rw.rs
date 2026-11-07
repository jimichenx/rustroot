fn main() {
    let mut n = 7;

    let r: &i32 = &n;
    println!("read via *r: {}", *r);

    let m: &mut i32 = &mut n;
    *m = *m + 1;
    println!("updated n:   {n}");
}
