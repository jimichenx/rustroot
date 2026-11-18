fn main() {
    // Built-in
    let arr: [i32; 3] = [1, 2, 3];
    let slice_ref: &[i32] = &arr;

    // Standard library
    let vec_owned: std::vec::Vec<i32> = vec![1, 2, 3];
    let maybe_num: std::option::Option<i32> = std::option::Option::Some(42);

    println!(
        "slice len: {}, vec len: {}, maybe: {:?}",
        slice_ref.len(),
        vec_owned.len(),
        maybe_num
    );
}
