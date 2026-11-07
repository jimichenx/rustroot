fn main() {
    assert_eq!(format!("{:<5}", "x"), "x    ");
    assert_eq!(format!("{:>5}", "x"), "    x");
    assert_eq!(format!("{:^5}", "x"), "  x  ");
    assert_eq!(format!("{:*<5}", "x"), "x****");
    assert_eq!(format!("{:+}", 42), "+42");
    assert_eq!(format!("{:05}", -42), "-0042");
    assert_eq!(format!("{:#06x}", 42), "0x002a");
    assert_eq!(format!("{:.2}", 1.234), "1.23");
    assert_eq!(format!("{:.3}", "Rust"), "Rus");
    assert_eq!(format!("{:.3}", 42), "42");
    assert_eq!(format!("{{{}}}", 42), "{42}");
}
