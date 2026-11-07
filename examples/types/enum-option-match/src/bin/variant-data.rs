//! Each variant carries its own data. A `Card` has a number; `Cash` has nothing.

enum Payment {
    Cash,
    Card(String),
}

fn describe(p: Payment) -> String {
    match p {
        Payment::Cash => String::from("cash"),
        Payment::Card(number) => format!("card {number}"),
    }
}

fn main() {
    println!("{}", describe(Payment::Cash));
    println!("{}", describe(Payment::Card(String::from("4242"))));
}
