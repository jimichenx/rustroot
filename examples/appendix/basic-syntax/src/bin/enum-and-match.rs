enum Status {
    Idle,
    Running(u32),
    Failed { code: i32 },
}

fn describe(state: &Status) {
    match state {
        Status::Idle => println!("idle"),
        Status::Running(pct) => println!("running: {pct}%"),
        Status::Failed { code } => println!("failed: {code}"),
    }
}

fn main() {
    let states = [
        Status::Idle,
        Status::Running(80),
        Status::Failed { code: 500 },
    ];
    for s in &states {
        describe(s);
    }

    if let Status::Running(pct) = states[1] {
        println!("current progress: {pct}%");
    }
}
