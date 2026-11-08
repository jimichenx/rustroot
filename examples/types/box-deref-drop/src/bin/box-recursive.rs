use std::mem::size_of;

// Uncomment `BadList` to trigger error[E0072]:
// recursive type `BadList` has infinite size.
// enum BadList {
//     Cons(i32, BadList),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn sum_list(list: &List) -> i32 {
    match list {
        List::Cons(val, next) => val + sum_list(next),
        List::Nil => 0,
    }
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("size of List = {} bytes", size_of::<List>());
    println!("sum of list = {}", sum_list(&list));
}
