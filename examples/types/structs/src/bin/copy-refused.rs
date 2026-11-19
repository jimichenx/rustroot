// Expect: E0204
#[derive(Clone)]
struct Named {
    name: String,
}

fn main() {
    let a = Named { name: String::from("Ferris") };
    let b = a.clone();
    println!("{} {}", a.name, b.name);
}
