//! Lesson 15.5: Encoding protocol state transitions at compile time with Typestate.
//! Uncomment the marked lines in `main` to verify `error[E0599]` and `error[E0382]`.

// ANCHOR: full
use std::marker::PhantomData;

// ANCHOR: state_defs
pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

pub struct DbClient<State> {
    endpoint: String,
    _state: PhantomData<State>,
}
// ANCHOR_END: state_defs

// ANCHOR: typestate_impls
impl DbClient<Disconnected> {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            _state: PhantomData,
        }
    }

    // Consumes `self` by value: old `Disconnected` binding is invalidated on transition
    pub fn connect(self) -> DbClient<Connected> {
        println!("[1/3] TCP connected -> {}", self.endpoint);
        DbClient {
            endpoint: self.endpoint,
            _state: PhantomData,
        }
    }
}

impl DbClient<Connected> {
    pub fn authenticate(self, token: &str) -> Result<DbClient<Authenticated>, &'static str> {
        if token.is_empty() {
            return Err("empty auth token");
        }
        println!("[2/3] Authenticated -> Ready for queries");
        Ok(DbClient {
            endpoint: self.endpoint,
            _state: PhantomData,
        })
    }
}

impl DbClient<Authenticated> {
    pub fn execute_query(&self, sql: &str) -> String {
        format!("[{}] `{sql}` -> 3 rows", self.endpoint)
    }

    // Consumes `self` on close, preventing any Use-After-Close at compile time
    pub fn close(self) {
        println!("[3/3] Connection closed cleanly");
    }
}
// ANCHOR_END: typestate_impls

fn main() {
    // Zero runtime overhead: PhantomData<State> adds 0 bytes to String
    assert_eq!(
        std::mem::size_of::<DbClient<Disconnected>>(),
        std::mem::size_of::<String>()
    );

    let raw = DbClient::<Disconnected>::new("postgres://127.0.0.1:5432");
    // raw.execute_query("SELECT 1"); // error[E0599]: no method named `execute_query` found for `DbClient<Disconnected>`

    let connected = raw.connect();
    let authed = connected.authenticate("secret-token").unwrap();

    println!("{}", authed.execute_query("SELECT id, email FROM accounts"));
    authed.close();
    // authed.execute_query("SELECT 1"); // error[E0382]: borrow of moved value: `authed`
}
// ANCHOR_END: full
