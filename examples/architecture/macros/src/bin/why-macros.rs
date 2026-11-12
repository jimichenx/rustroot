// ANCHOR: full
// ANCHOR: println_no_move
fn log_by_value(msg: String) {
    println!("[函数调用] {msg}");
}

macro_rules! inspect_borrow {
    ($val:expr) => {
        // Expand into a shared borrow (&$val) instead of moving $val
        match &$val {
            borrowed => println!("[自定义宏借用] {borrowed}"),
        }
    };
}
// ANCHOR_END: println_no_move

// ANCHOR: hygiene_isolation
macro_rules! add_with_temp {
    ($x:expr) => {{
        // `temp` inside the macro lives in its own lexical syntax context
        let temp = 100;
        $x + temp
    }};
}
// ANCHOR_END: hygiene_isolation

fn main() {
    // 1. Verify why println! and custom borrow macros do not move `s`
    let s = String::from("rust-root");
    println!("[标准库 println!] {}", s);
    inspect_borrow!(s);
    // `s` is still valid here because both macros only borrowed `&s`
    log_by_value(s);
    // Uncommenting the line below triggers error[E0382]: borrow of moved value: `s`
    // println!("{}", s);

    // 2. Verify lexical scope isolation (macro hygiene)
    let temp = 5;
    let total = add_with_temp!(temp * 2);
    println!("[作用域隔离] 调用方 temp = {temp}, 宏计算结果 = {total}");
}
// ANCHOR_END: full
