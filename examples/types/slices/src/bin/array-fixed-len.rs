// Uncomment the marked line to trigger error[E0308].
fn sum4(arr: &[i32; 4]) -> i32 {
    arr[0] + arr[1] + arr[2] + arr[3]
}

fn main() {
    let a4 = [10, 20, 30, 40];
    let a3 = [10, 20, 30];

    println!("sum4: {}, a3 len: {}", sum4(&a4), a3.len());
    // println!("sum3: {}", sum4(&a3));
}
