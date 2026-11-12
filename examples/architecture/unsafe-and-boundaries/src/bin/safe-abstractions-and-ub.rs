// Expect: E0499 (when uncommenting the naive safe split_at_mut implementation)
// ANCHOR: full
use std::slice;

/*
// 尝试只用 Safe Rust 把可变切片一分为二：
// 编译错误 E0499：cannot borrow `*values` as mutable more than once at a time
fn naive_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);
    (&mut values[..mid], &mut values[mid..])
}
*/

// ANCHOR: safe_split_at_mut
// 对外暴露 100% 安全的普通函数签名（头顶没有 unsafe）
pub fn my_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr: *mut i32 = values.as_mut_ptr();

    // 关键防线：在进入 unsafe 之前，用安全代码锁死前置边界条件。
    // 若外部传入越界的 mid（如 mid = 999），在此处安全触发 panic! 停机，绝不让越界指针流入下方
    assert!(mid <= len, "切分点 mid ({}) 超出切片长度 ({})", mid, len);

    // SAFETY:
    // 1. 上方的 assert!(mid <= len) 已经严格证明了 `0..mid` 与 `mid..len` 均落在原合法切片范围内；
    // 2. 左半区间 `[ptr, ptr + mid)` 与右半区间 `[ptr + mid, ptr + len)` 在物理内存上绝无重叠，
    //    满足两个 `&mut [i32]` 互不别名的排他公理；
    // 3. 返回的两个切片生命周期均自动绑定在输入参数 `values` 的借用期上。
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
// ANCHOR_END: safe_split_at_mut

// ANCHOR: mod_trust_boundary
// 演示为什么安全抽象的真实信任边界是整个 `mod` 模块：
pub mod trusted_buffer {
    pub struct FixedPair {
        data: [i32; 2],
        // 关键：active_len 必须是私有字段
        // 如果误标为 `pub active_len`，外部只需写一行安全的 `pair.active_len = 100`，
        // 就能直接击穿下方 get_unchecked 依赖的边界公理。
        active_len: usize,
    }

    impl FixedPair {
        pub fn new(first: i32) -> Self {
            Self {
                data: [first, 0],
                active_len: 1,
            }
        }

        pub fn push_second(&mut self, second: i32) {
            self.data[1] = second;
            self.active_len = 2;
        }

        pub fn last_valid(&self) -> i32 {
            // SAFETY: 模块内部所有方法均严格维护了 `1 <= active_len <= 2` 的不变量，
            // 且外部模块无法直接篡改私有字段 active_len，因此下标 `active_len - 1` 永不越界。
            unsafe { *self.data.get_unchecked(self.active_len - 1) }
        }
    }
}
// ANCHOR_END: mod_trust_boundary

fn main() {
    let mut scores = [10, 20, 30, 40, 50, 60];

    // 调用方无需编写任何 unsafe，即可同时拿到左右两半可变切片并行修改
    let (left, right) = my_split_at_mut(&mut scores, 2);
    left[0] += 5;
    right[3] += 100;
    println!("[安全抽象 my_split_at_mut] 修改后的数组: {:?}", scores);

    // 验证模块级信任边界保护下的容器操作
    let mut pair = trusted_buffer::FixedPair::new(42);
    println!("[模块边界保护] 初始末尾有效值 = {}", pair.last_valid());
    pair.push_second(99);
    println!("[模块边界保护] 追加后末尾有效值 = {}", pair.last_valid());
}
// ANCHOR_END: full
