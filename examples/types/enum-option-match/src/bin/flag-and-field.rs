#[allow(dead_code)]
struct BadRole {
    role_tag: u8,
    vip_points: u64,
    dept_id: u64,
    security_key: u64,
}

fn main() {
    let user = BadRole {
        role_tag: 1,
        vip_points: 500,
        dept_id: 0,
        security_key: 9999,
    };

    if user.role_tag == 1 {
        println!(
            "member points = {}, leaked security_key = {}",
            user.vip_points, user.security_key
        );
    }
}
