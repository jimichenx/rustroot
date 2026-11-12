//! Reproducing and fixing error[E0597]: borrowed value does not live long enough.
//! Uncomment the broken block to trigger error[E0597].

// ANCHOR: full
fn build_endpoint(host: &str, port: u16) -> String {
    // Returning owned `String` transfers heap ownership to the caller cleanly,
    // avoiding dangling borrows to stack-local temporaries (error[E0515]).
    format!("{host}:{port}")
}

fn main() {
    // Broken pattern: outer reference outlives inner owner
    // let view: &str;
    // {
    //     let raw = build_endpoint("127.0.0.1", 8080);
    //     view = &raw; // error[E0597]: `raw` does not live long enough
    // }
    // println!("{view}");

    // Fix: align owner scope so the owner outlives all borrows
    let raw = build_endpoint("127.0.0.1", 8080);
    let view: &str = &raw;
    println!("listening on {view}");
}
// ANCHOR_END: full
