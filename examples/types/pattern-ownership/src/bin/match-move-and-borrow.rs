// Expect: E0382
#[allow(dead_code)]
#[derive(Debug)]
enum Task {
    Run(String),
    Stop,
}

fn main() {
    let task = Task::Run(String::from("backup-db"));

    match &task {
        Task::Run(name) => println!("inspecting: {name}"),
        Task::Stop => println!("inspecting: stop"),
    }

    match task {
        Task::Run(name) => println!("consuming: {name}"),
        Task::Stop => println!("consuming: stop"),
    }

    // println!("after move: {task:?}");
}
