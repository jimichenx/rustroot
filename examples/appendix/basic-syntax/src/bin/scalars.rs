use std::mem::size_of;

fn main() {
    let count: usize = 3;
    let flag: bool = true;
    let letter: char = '中';
    println!("{count} {flag} {letter}");
    println!(
        "i32={} char={} bool={}",
        size_of::<i32>(),
        size_of::<char>(),
        size_of::<bool>()
    );
}
