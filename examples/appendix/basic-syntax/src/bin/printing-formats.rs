fn main() {
    let price = 12.3456;
    let count = 42;

    println!("{price:.2}");
    println!("|{count:>5}|");
    println!("|{count:05}|");
    println!("{count:b} {count:#x}");
}
