fn main() {
    let good = vec![104, 105];
    let bad = vec![0xff, 0xfe];

    println!("{}", String::from_utf8(good).is_ok());
    println!("{}", String::from_utf8(bad).is_ok());
}
