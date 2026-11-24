struct User {
    name: Option<String>,
    nickname: Option<String>,
}

fn main() {
    let mut u = User {
        name: Some(String::from("Ferris")),
        nickname: None,
    };

    let display = u.nickname.as_ref().map(|s| s.as_str()).unwrap_or("anonymous");
    println!("display = {display}");

    let peek: Option<&String> = u.name.as_ref();
    println!("peek = {peek:?}");

    let taken: Option<String> = u.name.take();
    println!("taken = {taken:?}, remaining u.name = {:?}", u.name);
}
