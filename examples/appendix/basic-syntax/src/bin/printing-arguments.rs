fn main() {
    let name = "Rust";
    let width: usize = 5;
    let precision: usize = 2;

    assert_eq!(format!("{} {}", "a", "b"), "a b");
    assert_eq!(format!("{1} {0} {1}", "a", "b"), "b a b");
    assert_eq!(format!("{name} {n}", n = 3), "Rust 3");
    assert_eq!(format!("{0} {}", "a"), "a a");
    assert_eq!(format!("{:>width$}", "x"), "    x");
    assert_eq!(format!("{:>1$}", "x", 5), "    x");
    assert_eq!(format!("{:.precision$}", 1.234), "1.23");
    assert_eq!(format!("{:.*}", 2, 1.234), "1.23");
}
