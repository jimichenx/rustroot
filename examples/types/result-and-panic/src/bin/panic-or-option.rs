fn main() {
    let v = vec![10, 20, 30];
    let i = 10;

    println!("{:?}", v.get(i));
    // println!("{}", v[i]);

    let n: i32 = "42".parse().expect("a literal we wrote ourselves");
    println!("{n}");
}
