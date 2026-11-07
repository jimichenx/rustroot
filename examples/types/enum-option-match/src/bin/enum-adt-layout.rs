//! Compare the memory sizes of a C-like enum, a struct (BadRole), and a payload enum (Role).

use std::mem::{size_of, size_of_val};

#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[allow(dead_code)]
struct BadRole {
    role_tag: u8,
    vip_points: u64,
    dept_id: u64,
    security_key: u64,
}

#[allow(dead_code)]
enum Role {
    Guest,
    Member(u64),
    Admin { dept_id: u64, security_key: u64 },
}

fn main() {
    let r1 = Role::Guest;
    let r2 = Role::Member(500);
    let r3 = Role::Admin {
        dept_id: 1,
        security_key: 9999,
    };

    println!(
        "Direction: {}B, BadRole (struct): {}B, Role (enum): {}B",
        size_of::<Direction>(),
        size_of::<BadRole>(),
        size_of::<Role>()
    );
    println!(
        "r1 = {}B, r2 = {}B, r3 = {}B",
        size_of_val(&r1),
        size_of_val(&r2),
        size_of_val(&r3)
    );
}
