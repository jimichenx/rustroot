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
