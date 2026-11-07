//! Rust's native sum type (`enum`): closed set of variants, inline fixed-size memory,
//! and compile-time exhaustiveness check (uncomment the partial match to see E0004).

use std::mem::size_of;

enum Role {
    Guest,
    Member(u64),
    Admin { dept_id: u64, security_key: u64 },
}

fn describe(role: &Role) -> u64 {
    match role {
        Role::Guest => 0,
        Role::Member(points) => *points,
        Role::Admin { dept_id, security_key } => dept_id + security_key,
    }
}

fn main() {
    let roles: [Role; 3] = [
        Role::Guest,
        Role::Member(500),
        Role::Admin {
            dept_id: 1,
            security_key: 9999,
        },
    ];

    println!("size_of::<Role>() = {} bytes", size_of::<Role>());
    println!("size_of::<[Role; 3]>() = {} bytes", size_of::<[Role; 3]>());
    for r in &roles {
        println!("value = {}", describe(r));
    }
}
