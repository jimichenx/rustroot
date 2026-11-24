enum Role {
    Guest,
    Member(u64),
    Admin { dept_id: u64, security_key: u64 },
}

// ANCHOR: let_else
fn verify_admin(role: &Role) -> u64 {
    let Role::Admin { dept_id, security_key } = role else {
        return 0;
    };
    dept_id + security_key
}
// ANCHOR_END: let_else

fn main() {
    // ANCHOR: if_let
    let user = Role::Member(500);
    if let Role::Member(points) = &user {
        println!("member bonus points = {points}");
    }
    // ANCHOR_END: if_let

    let admin = Role::Admin {
        dept_id: 1,
        security_key: 9999,
    };
    println!("guest check = {}", verify_admin(&Role::Guest));
    println!("admin check = {}", verify_admin(&admin));
}
