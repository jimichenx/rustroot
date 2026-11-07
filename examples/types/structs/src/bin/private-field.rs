//! Outside its module, a private field can be neither read nor written.
//! The only way to get an `Even` is through a function inside the module,
//! so the value inside stays even.
//! Uncomment the last line to get error[E0451]: field `value` of struct `Even` is private.

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
