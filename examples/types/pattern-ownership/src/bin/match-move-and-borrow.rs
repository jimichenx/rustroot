//! Ownership in pattern matching: matching `&task` borrows the payload (`&String`),
//! whereas matching `task` by value moves the `String` out (uncomment the last line for E0382).

#[allow(dead_code)]
#[derive(Debug)]
enum Task {
    Run(String),
    Stop,
}

fn main() {
    let task = Task::Run(String::from("backup-db"));

    // 1. Match on a reference `&task`: `name` is `&String`, `task` stays valid.
    match &task {
        Task::Run(name) => println!("inspecting: {name}"),
        Task::Stop => println!("inspecting: stop"),
    }

    // 2. Match by value `task`: the `String` moves into `name`.
    match task {
        Task::Run(name) => println!("consuming: {name}"),
        Task::Stop => println!("consuming: stop"),
    }

    // Uncomment to observe error[E0382]: borrow of partially moved value: `task`
    // println!("after move: {task:?}");
}
