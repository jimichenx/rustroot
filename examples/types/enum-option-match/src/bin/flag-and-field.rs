//! Using a struct with a tag to represent mutually exclusive user roles.
//! Guest, Member, and Admin carry different data, but the struct forces every
//! instance to carry all fields — and nothing stops reading the wrong field.

#[allow(dead_code)]
struct BadRole {
    role_tag: u8,      // 0: Guest, 1: Member, 2: Admin
    vip_points: u64,   // only meaningful for Member
    dept_id: u64,      // only meaningful for Admin
    security_key: u64, // only meaningful for Admin
}

fn main() {
    // A regular member (role_tag = 1): yet we still must fill in Admin fields!
    let user = BadRole {
        role_tag: 1,
        vip_points: 500,
        dept_id: 0,
        security_key: 9999,
    };

    if user.role_tag == 1 {
        // Nothing stops us from reading Admin's security_key on a regular Member:
        println!(
            "member points = {}, leaked security_key = {}",
            user.vip_points, user.security_key
        );
    }
}
