fn main() {
    let mut nums: [i32; 4] = [40, 10, 30, 20];

    let s: &[i32] = &nums;
    println!("len: {}, is_empty: {}, contains 30: {}", s.len(), s.is_empty(), s.contains(&30));

    let m: &mut [i32] = &mut nums[1..4];
    m.reverse();
    println!("after reversing [1..4]: {:?}", nums);
}
