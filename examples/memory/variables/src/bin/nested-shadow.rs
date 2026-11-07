fn main() {
    let n = 7;
    {
        let n = n + 1;
        println!("{n}");
    }
    println!("{n}");
}
