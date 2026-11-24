// Expect: E0005
struct User {
    name: &'static str,
    age: u32,
}

#[allow(dead_code)]
enum Role {
    Guest,
    Member(u64),
    Admin { dept_id: u64, security_key: u64 },
}

fn main() {
    let (x, y) = (10, 20);
    let User { name, age } = User { name: "Ferris", age: 7 };
    println!("point = ({x}, {y}), user = {name} ({age})");

    let _role = Role::Member(500);
    // let Role::Member(points) = _role;
}
