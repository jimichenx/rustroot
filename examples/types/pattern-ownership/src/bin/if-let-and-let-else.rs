//! Single-variant ergonomics: `if let` for optional side actions,
//! and `let ... else` for early-return guards that keep the main flow flat.

enum Role {
    Guest,
    Member(u64),
    Admin { dept_id: u64, security_key: u64 },
}

fn verify_admin(role: &Role) -> u64 {
    let Role::Admin { dept_id, security_key } = role else {
        return 0;
    };
    // `dept_id` and `security_key` are available in the outer scope without nesting!
    dept_id + security_key
}

fn main() {
    let user = Role::Member(500);
    if let Role::Member(points) = &user {
        println!("member bonus points = {points}");
    }

    let admin = Role::Admin {
        dept_id: 1,
        security_key: 9999,
    };
    println!("guest check = {}", verify_admin(&Role::Guest));
    println!("admin check = {}", verify_admin(&admin));
}
