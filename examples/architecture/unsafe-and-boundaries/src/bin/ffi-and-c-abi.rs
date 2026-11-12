// ANCHOR: full
use std::ffi::{CStr, CString, c_char, c_int};
use std::mem::{offset_of, size_of};

// ANCHOR: rust_calls_c
// 方向一（向内借力）：声明操作系统自带的 C 语言标准库函数（Rust 2024 语法要求显式标明 unsafe extern "C"）
unsafe extern "C" {
    fn abs(input: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
}

// 按照 16.2 节的「安全抽象」原则，把底层的 C 语言 unsafe 调用封装成 100% 安全的普通 Rust 函数
pub fn safe_c_abs(n: i32) -> Option<i32> {
    // 拦截 C 语言 abs(i32::MIN) 会发生有符号整数溢出（在 C 中属于 UB）的边界隐患
    if n == i32::MIN {
        return None;
    }
    // SAFETY: 已排除唯一溢出的 i32::MIN，C 标准库 abs 对其余所有 32 位整数均为纯计算且无内存副作用
    Some(unsafe { abs(n) })
}
// ANCHOR_END: rust_calls_c

// ANCHOR: repr_c_layout
// 默认 Rust 内存布局（repr(Rust)）：编译器为了节省对齐空洞（Padding），会自动重排字段顺序
struct DefaultRustPacket {
    flag_a: u8,  // 1 字节
    payload: u32,// 4 字节（编译器通常会把它排到最前面，让两个 u8 紧挨在后面，总大小压缩为 8 字节）
    flag_b: u8,  // 1 字节
}

// C 语言兼容布局（#[repr(C)]）：严格保持源码声明的字段先后顺序，与 C 语言 struct 字节偏移 100% 对齐
#[repr(C)]
pub struct CCompatiblePoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
struct CLayoutPacket {
    flag_a: u8,  // 偏移 +0（后跟 3 字节填充空洞以对齐 4 字节的 payload）
    payload: u32,// 偏移 +4
    flag_b: u8,  // 偏移 +8（后跟 3 字节尾部填充，总大小变为 12 字节）
}
// ANCHOR_END: repr_c_layout

// ANCHOR: export_rust_to_c
// 方向二（向外赋能）：将 Rust 函数按 C ABI 导出，供 Python (ctypes) / Node.js / C / Java 调用
// - `#[unsafe(no_mangle)]`：禁止编译器混淆改写函数符号名，保留原名 `rust_distance_squared`
// - `pub extern "C"`：指示编译器生成遵循平台标准 C 调用约定的机器码入口
#[unsafe(no_mangle)]
pub extern "C" fn rust_distance_squared(p: CCompatiblePoint) -> f64 {
    p.x * p.x + p.y * p.y
}
// ANCHOR_END: export_rust_to_c

fn main() {
    // 1. 验证调用封装后的外部 C 标准库函数
    println!("[Rust 调 C] safe_c_abs(-42) = {:?}", safe_c_abs(-42));

    // 演示跨语言字符串转换：Rust 字符串（指针 + 长度，中间可含 0） <-> C 字符串（以末尾 `\0` 字节结束）
    let rust_msg = CString::new("Hello across FFI boundary").unwrap();
    // SAFETY: CString 保证了末尾必然带有合法的 `\0` 终止符，且指针在当前作用域内有效
    let c_len = unsafe { strlen(rust_msg.as_ptr()) };
    let back_to_rust: &CStr = rust_msg.as_c_str();
    println!(
        "[C 字符串桥接] 内容 = {:?}, C strlen 测得字节长度 = {}",
        back_to_rust, c_len
    );

    // 2. 验证默认 Rust 布局重排 vs #[repr(C)] 固定顺序布局的物理差异
    println!(
        "\n[内存布局对比] DefaultRustPacket 大小 = {} 字节 (flag_a 偏移={}, payload 偏移={}, flag_b 偏移={})",
        size_of::<DefaultRustPacket>(),
        offset_of!(DefaultRustPacket, flag_a),
        offset_of!(DefaultRustPacket, payload),
        offset_of!(DefaultRustPacket, flag_b),
    );
    println!(
        "[内存布局对比] #[repr(C)] Packet 大小 = {} 字节 (flag_a 偏移={}, payload 偏移={}, flag_b 偏移={})",
        size_of::<CLayoutPacket>(),
        offset_of!(CLayoutPacket, flag_a),
        offset_of!(CLayoutPacket, payload),
        offset_of!(CLayoutPacket, flag_b),
    );

    // 3. 验证导出的 C ABI 函数
    let pt = CCompatiblePoint { x: 3.0, y: 4.0 };
    println!(
        "\n[导出 C ABI 函数] rust_distance_squared(3.0, 4.0) = {}",
        rust_distance_squared(pt)
    );
}
// ANCHOR_END: full
