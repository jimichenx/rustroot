//! Reproducing and fixing error[E0277]: `Rc<T>` does not implement `Send`.
//! Uncomment the Rc block to trigger error[E0277].

// ANCHOR: full
use std::sync::Arc;
use std::thread;

fn main() {
    // Broken pattern: Rc uses non-atomic reference counting, so it is !Send
    // let rc_cfg = std::rc::Rc::new(String::from("prod-eu"));
    // let handle = thread::spawn(move || {
    //     println!("{rc_cfg}"); // error[E0277]: `Rc<String>` cannot be sent between threads safely
    // });

    // Fix: use Arc<T> for cross-thread shared ownership
    let shared_cfg = Arc::new(String::from("prod-eu"));
    let worker_cfg = Arc::clone(&shared_cfg);

    let handle = thread::spawn(move || format!("worker region = {worker_cfg}"));

    println!("main region = {shared_cfg}");
    println!("{}", handle.join().unwrap());
}
// ANCHOR_END: full
