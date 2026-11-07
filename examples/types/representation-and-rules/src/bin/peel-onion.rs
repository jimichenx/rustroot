use std::mem::size_of;

fn first_positive(nums: &[i32]) -> Option<&i32> {
    for n in nums {
        if *n > 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    let data: [i32; 4] = [-3, 0, 7, 12];
    let slice: &[i32] = &data;
    let found: Option<&i32> = first_positive(slice);

    println!("found: {:?}", found);
    println!("size of [i32; 4]:     {} bytes", size_of::<[i32; 4]>());
    println!("size of &[i32]:       {} bytes", size_of::<&[i32]>());
    println!("size of Option<&i32>: {} bytes", size_of::<Option<&i32>>());
}
