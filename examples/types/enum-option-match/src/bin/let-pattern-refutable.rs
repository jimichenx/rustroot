//! `let PATTERN = EXPR;`: product types (tuple, struct) have only one possibility,
//! so a plain `let` destructures them infallibly (irrefutable pattern).
//! A sum type (`enum`) has multiple possibilities: uncomment line 24 to observe E0005.

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
    // 1. Product types: only 1 possibility -> plain `let` always succeeds.
    let (x, y) = (10, 20);
    let User { name, age } = User { name: "Ferris", age: 7 };
    println!("point = ({x}, {y}), user = {name} ({age})");

    // 2. Sum type: 3 possibilities -> plain `let` may fail!
    let _role = Role::Member(500);
    // Uncomment to observe error[E0005]: refutable pattern in local binding
    // let Role::Member(points) = _role;
}
