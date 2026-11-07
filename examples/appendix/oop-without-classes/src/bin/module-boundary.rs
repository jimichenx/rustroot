//! Uncomment the marked line and compile to get error[E0616].

mod accounts {
    pub struct Account {
        // Private to this module: no `pub`.
        balance: i64,
    }

    impl Account {
        pub fn new(balance: i64) -> Self {
            Self { balance }
        }

        pub fn balance(&self) -> i64 {
            self.balance
        }
    }

    // A free function in the same module can read the private field,
    // so this code does not have to live in an `impl` block.
    pub fn is_overdrawn(account: &Account) -> bool {
        account.balance < 0
    }
}

fn main() {
    let account = accounts::Account::new(-5);

    println!("overdrawn: {}", accounts::is_overdrawn(&account));
    println!("balance: {}", account.balance());

    // Outside the module the field is unreachable:
    // println!("{}", account.balance); // error[E0616]
}
