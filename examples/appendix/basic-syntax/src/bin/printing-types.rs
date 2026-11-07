fn main() {
    let number = 42;
    assert_eq!(format!("{}", number), "42");
    assert_eq!(format!("{:?}", [1, 2]), "[1, 2]");
    assert_eq!(format!("{:x?}", [15, 26]), "[f, 1a]");
    assert_eq!(format!("{:X?}", [15, 26]), "[F, 1A]");
    assert_eq!(format!("{:b}", number), "101010");
    assert_eq!(format!("{:o}", number), "52");
    assert_eq!(format!("{:x} {:X}", 255, 255), "ff FF");
    assert_eq!(format!("{:e} {:E}", 12.5, 12.5), "1.25e1 1.25E1");
    assert!(format!("{:p}", &number).starts_with("0x"));
}
