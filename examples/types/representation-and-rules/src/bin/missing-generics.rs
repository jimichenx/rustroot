// Uncomment the marked line to trigger error[E0107].
fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2: Vec<String> = Vec::new();
    // let v3: Vec = Vec::new();
    println!("{} {}", v1.len(), v2.len());
}
