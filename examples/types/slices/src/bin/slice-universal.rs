fn sum(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &x in slice {
        total += x;
    }
    total
}

fn main() {
    let stack_arr: [i32; 4] = [10, 20, 30, 40];
    let heap_vec: Vec<i32> = vec![100, 200, 300];

    println!("from array:     {}", sum(&stack_arr));
    println!("from array[1..]:{}", sum(&stack_arr[1..3]));
    println!("from Vec:       {}", sum(&heap_vec));
    println!("from Vec[..2]:  {}", sum(&heap_vec[..2]));
}
