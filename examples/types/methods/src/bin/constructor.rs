//! An associated function with no receiver acts as the constructor.
//! With the field private, `from_half` is the only way in, so `value` stays even.

mod parity {
    pub struct Even {
        value: u32,
    }

    impl Even {
        pub fn from_half(half: u32) -> Even {
            Even { value: half * 2 }
        }
        pub fn value(&self) -> u32 {
            self.value
        }
    }
}

fn main() {
    let e = parity::Even::from_half(21);
    println!("{}", e.value());
}
