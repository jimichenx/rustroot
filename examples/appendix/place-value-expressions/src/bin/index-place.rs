fn main() {
    let mut values = vec![1, 2, 3];
    values[0] = 10;

    let first = &mut values[1];
    *first += 20;

    println!("{values:?}");
}
