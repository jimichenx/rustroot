// Uncomment the marked line to trigger error[E0308].
struct User {
    id: i32,
}

fn main() {
    let mut u: User = User { id: 1 };
    let r: &User = &u;
    println!("{}", r.id);

    let m: &mut User = &mut u;
    m.id = 2;
    // let r2: &mut User = &u;
    println!("{}", m.id);
}
