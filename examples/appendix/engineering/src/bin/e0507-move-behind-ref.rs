//! Reproducing and fixing error[E0507]: cannot move out of a shared reference.
//! Uncomment the marked line to trigger error[E0507].

// ANCHOR: full
struct Account {
    email: Option<String>,
}

fn domain_of(account: &Account) -> Option<&str> {
    // `account.email` is Option<String>, which does not implement Copy.
    // Moving it out of `&Account` would leave `*account` partially uninitialized:
    // let Some(addr) = account.email else { return None; }; // error[E0507]

    // Fix: borrow inside Option via `.as_deref()` (`&Option<String>` -> `Option<&str>`)
    let addr = account.email.as_deref()?;
    addr.split_once('@').map(|(_, domain)| domain)
}

fn main() {
    let acc = Account {
        email: Some(String::from("dev@rustroot.cn")),
    };
    println!("domain: {:?}", domain_of(&acc));
    println!("original email intact: {:?}", acc.email);
}
// ANCHOR_END: full
