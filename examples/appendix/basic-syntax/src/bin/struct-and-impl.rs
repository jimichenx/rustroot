struct User {
    name: String,
    active: bool,
}

impl User {
    fn new(name: String) -> Self {
        Self { name, active: true }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

fn main() {
    let u = User::new(String::from("Alice"));
    println!("{} {}", u.name, u.is_active());
}
