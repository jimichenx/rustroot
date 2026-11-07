//! Breaking out of `for x in v` drops the elements still inside the
//! iterator (`IntoIter` owns them), not the ones already taken out.

struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("dropped {}", self.0);
    }
}

fn main() {
    let v = vec![Noisy("a"), Noisy("b"), Noisy("c"), Noisy("d")];
    let mut taken = Vec::new();
    for item in v {
        if item.0 == "c" {
            break; // c and d never leave the iterator
        }
        taken.push(item); // a and b now live in `taken`
    }
    println!("loop done");
}
