pub struct Account {
    balance: i64,
}

impl Account {
    fn new(balance: i64) -> Self {
        Self { balance }
    }

    // Read-only: the receiver is a shared borrow.
    fn balance(&self) -> i64 {
        self.balance
    }

    // Mutation: an exclusive borrow, visible in the signature.
    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }

    // Consuming: the receiver is moved in; the account is gone after this.
    fn close(self) -> i64 {
        self.balance
    }
}

fn main() {
    let mut account = Account::new(100);
    account.deposit(50);
    println!("{}", account.balance());
    println!("closed with {}", account.close());
}
