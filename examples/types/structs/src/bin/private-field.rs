// Expect: E0451
mod parity {
    pub struct Even {
        value: u32,
    }

    pub fn double(half: u32) -> Even {
        Even { value: half * 2 }
    }

    pub fn value(e: &Even) -> u32 {
        e.value
    }
}

fn main() {
    let e = parity::double(21);
    println!("{}", parity::value(&e));
    // let odd = parity::Even { value: 3 };
}
