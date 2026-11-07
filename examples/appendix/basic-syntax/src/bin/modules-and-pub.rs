mod auth {
    pub struct Token {
        pub id: u32,
        secret: u32,
    }

    impl Token {
        pub fn new(id: u32, secret: u32) -> Self {
            Self { id, secret }
        }

        pub fn verify(&self, guess: u32) -> bool {
            self.secret == guess
        }
    }
}

use auth::Token;

fn main() {
    let t = Token::new(1, 999);
    // println!("{}", t.secret); // error[E0616]: field `secret` of struct `Token` is private
    println!("id={} ok={}", t.id, t.verify(999));
}
