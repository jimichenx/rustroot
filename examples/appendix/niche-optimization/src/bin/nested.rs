//! Nesting does not refill seats: the inner Option may consume the only gap.

use std::mem::size_of;

fn main() {
    println!("Option<i32>           {}", size_of::<Option<i32>>());
    println!("Option<Option<i32>>   {}", size_of::<Option<Option<i32>>>());
    println!("Option<Option<bool>>  {}", size_of::<Option<Option<bool>>>());
    println!("Option<&i32>          {}", size_of::<Option<&'static i32>>());
    println!("Option<Option<&i32>>  {}", size_of::<Option<Option<&'static i32>>>());
}
