//! Destructuring `Role` with `match` and checking exhaustiveness.
//! Comment out the `Role::Admin` arm to observe error[E0004]: non-exhaustive patterns.

enum Role {
    Guest,
    Member(u64),
    Admin { dept_id: u64, security_key: u64 },
}

fn quota(role: &Role) -> u64 {
    match role {
        Role::Guest => 10,
        Role::Member(points) => 100 + points,
        Role::Admin { dept_id, security_key } => dept_id * 1000 + security_key,
    }
}

fn main() {
    let r1 = Role::Guest;
    let r2 = Role::Member(500);
    let r3 = Role::Admin {
        dept_id: 2,
        security_key: 9999,
    };

    println!(
        "quotas: {}, {}, {}",
        quota(&r1),
        quota(&r2),
        quota(&r3)
    );
}
