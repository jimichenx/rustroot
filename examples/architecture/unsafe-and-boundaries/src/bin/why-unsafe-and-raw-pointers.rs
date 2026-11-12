// Expect: E0133 (when uncommenting dereferencing raw pointers outside an unsafe block)
// ANCHOR: full

// ANCHOR: unsafe_fn_demo
// 定义一个 unsafe 函数：调用方在调用前必须自行担保传入的指针非空且指向合法对齐的 u32 内存
unsafe fn write_hardware_register(reg_ptr: *mut u32, mask: u32) {
    // Rust 2024 要求在 unsafe fn 内部也显式写出 unsafe {} 块，标清真正危险的那一行操作
    unsafe {
        *reg_ptr |= mask;
    }
}
// ANCHOR_END: unsafe_fn_demo

fn main() {
    // ANCHOR: create_raw_pointers
    let mut counter: i32 = 10;

    // 1. 在 100% 安全的普通 Rust 代码中直接创建裸指针
    //    使用 `&raw const` 与 `&raw mut`（或 `as *const i32`）获取内存地址：
    //    注意：这里同时并存着指向同一块内存的只读裸指针 r1 与可变裸指针 r2，编译器完全放行
    let r1: *const i32 = &raw const counter;
    let r2: *mut i32 = &raw mut counter;

    println!(
        "[安全代码] 成功创建两个指向同一地址的裸指针: r1 = {:p}, r2 = {:p}",
        r1, r2
    );
    // ANCHOR_END: create_raw_pointers

    /*
    // 如果在安全代码中直接对裸指针执行 `*r1` 解引用读取内存：
    // 编译错误 E0133：dereference of raw pointer is unsafe and requires unsafe function or block
    println!("val = {}", *r1);
    */

    // ANCHOR: deref_in_unsafe
    // 2. 只有真正开门读写内存（解引用 `*r1` / `*r2`）的那一刻，才必须写进 `unsafe {}` 块
    // SAFETY: r1 和 r2 均直接取自当前栈帧上存活的局部变量 counter，地址非空、已对齐且无并发竞争
    unsafe {
        *r2 += 5;
        println!("[unsafe 解引用] 通过 *r2 修改后，通过 *r1 读出值 = {}", *r1);
    }
    // ANCHOR_END: deref_in_unsafe

    // 3. 调用 unsafe fn 同样必须放入 unsafe 块中，由调用方背书前置条件
    let mut simulated_reg: u32 = 0b0001;
    let reg_addr: *mut u32 = &raw mut simulated_reg;
    // SAFETY: reg_addr 指向当前栈上有效初始化的 u32 变量
    unsafe {
        write_hardware_register(reg_addr, 0b0100);
    }
    println!("[unsafe 函数调用] 寄存器写入掩码后的值 = {:#06b}", simulated_reg);

    /*
    // 破除头号误区：unsafe {} 绝不会关掉借用检查器！
    // 在 unsafe 块内部，普通引用 &T 和 &mut T 依然 100% 遵守读写排他铁律：
    unsafe {
        let mut s = String::from("rust");
        let ref1 = &s;
        let ref2 = &mut s; // 照样报编译错误 E0502：cannot borrow `s` as mutable
        println!("{}, {}", ref1, ref2);
    }
    */
}
// ANCHOR_END: full
