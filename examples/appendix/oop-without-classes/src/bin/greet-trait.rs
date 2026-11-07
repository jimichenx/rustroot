trait Greet {
    // Implementors must provide `name`.
    fn name(&self) -> String;

    // Shared flow: the default method calls back into `name`.
    fn greet(&self) -> String {
        format!("Hi, {}", self.name())
    }
}

struct User {
    name: String,
}

struct Bot {
    id: u32,
}

impl Greet for User {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Greet for Bot {
    fn name(&self) -> String {
        format!("bot-{}", self.id)
    }
}

fn main() {
    let user = User { name: String::from("Alice") };
    let bot = Bot { id: 7 };
    // One `greet`, two bindings of `name`.
    println!("{}", user.greet());
    println!("{}", bot.greet());
}
