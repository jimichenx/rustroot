fn main() {
    let mut s = String::from("first");
    let t = s;
    s = String::from("second");
    println!("{s}");
    {
        let r = &t;
        let s = 7;
        println!("{s} {r}");
    }
    s = t;
    let s = s;
    println!("{s}");
}
