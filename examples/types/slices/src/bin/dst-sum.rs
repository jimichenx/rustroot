// Uncomment the marked function to trigger error[E0277].
/*
fn bad_sum(arr: [i32]) -> i32 {
    0
}
*/

fn sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for &x in arr {
        total += x;
    }
    total
}

fn main() {
    let a4: [i32; 4] = [10, 20, 30, 40];
    let a3: [i32; 3] = [10, 20, 30];

    println!("sum of a4:       {}", sum(&a4));
    println!("sum of a3:       {}", sum(&a3));
    println!("sum of a4[1..3]: {}", sum(&a4[1..3]));
}
