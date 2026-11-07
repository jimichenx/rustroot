//! Expected error when uncommented: E0573 (expected type, found variant `Role::Member`)
//! Only the outer `Role` is a compile-time type entity; variants are constructors, not types.

#[allow(dead_code)]
enum Role {
    Guest,
    Member(u64),
    Admin { dept_id: u64, security_key: u64 },
}

// Uncomment to observe error[E0573]:
// fn check_member(m: Role::Member) {}

fn check_role(_r: Role) {}

fn main() {
    let r1: Role = Role::Guest;
    let r2: Role = Role::Member(500);
    let r3: Role = Role::Admin {
        dept_id: 1,
        security_key: 9999,
    };

    check_role(r1);
    check_role(r2);
    check_role(r3);
}
