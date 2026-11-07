struct Pair(String, String);

fn main() {
    let pair = Pair(String::from("left"), String::from("right"));
    let left = pair.0;

    println!("{left}");
    println!("{}", pair.1);
}
