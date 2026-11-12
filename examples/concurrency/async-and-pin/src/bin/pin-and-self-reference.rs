// Expect: E0277 (when uncommenting std::mem::swap on pinned !Unpin futures)
// ANCHOR: full
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// ANCHOR: self_ref_sim
// 模拟编译器为跨 .await 借用局部变量的 async fn 生成的自引用状态机：
// 包含内联数据数组 buf，以及指向同结构体内部 buf 字段的指针 slice_ptr
struct SelfRefStateMachine {
    buf: [u8; 4],
    slice_ptr: *const u8,
    // PhantomPinned 显式将该结构体标记为 !Unpin（禁止解除地址固定），
    // 与编译器自动为自引用 async fn 状态机生成的标记完全一致
    _pin: PhantomPinned,
}

impl SelfRefStateMachine {
    // 阶段 0：仅构造初始状态，此时尚未建立内部自引用，允许任意 Move
    fn new(data: [u8; 4]) -> Self {
        Self {
            buf: data,
            slice_ptr: std::ptr::null(),
            _pin: PhantomPinned,
        }
    }

    // 模拟第一次 poll：状态机开始运行，内部字段 slice_ptr 指向同结构体的 buf 字段
    fn poll_first_step(self: Pin<&mut Self>) {
        let buf_addr = self.buf.as_ptr();
        // SAFETY: 我们仅将内部指针设置为当前已固定对象的字段地址，绝不将对象从内存中移走
        unsafe {
            let this = self.get_unchecked_mut();
            this.slice_ptr = buf_addr;
        }
    }

    fn inspect_addresses(&self, label: &str) {
        println!(
            "[{}] 当前 buf 实际地址 = {:p} | slice_ptr 内部记录的地址 = {:p} | 地址是否一致: {}",
            label,
            self.buf.as_ptr(),
            self.slice_ptr,
            std::ptr::eq(self.buf.as_ptr(), self.slice_ptr)
        );
    }
}
// ANCHOR_END: self_ref_sim

// ANCHOR: real_async_pin
// 真实的 async fn：跨过 .await 借用局部栈数组，编译器自动将其展开为 !Unpin 的自引用状态机
async fn parse_packet_header() -> u8 {
    let buf: [u8; 4] = [104, 105, 33, 0];
    let first_byte_ref: &u8 = &buf[0]; // 借用同状态机内部的局部数组字段

    // 跨越挂起点：buf 与 first_byte_ref 被一起打包进同一个状态机变体中
    std::future::ready(()).await;

    *first_byte_ref
}
// ANCHOR_END: real_async_pin

fn main() {
    println!("=== 1. 阶段 0（尚未建立内部自引用前）：状态机可以自由 Move ===");
    let raw_state = SelfRefStateMachine::new([10, 20, 30, 40]);
    let moved_state = raw_state; // 此时 slice_ptr 尚未指向自身，按位 Move 完全安全

    println!("\n=== 2. 进入首次 poll 建立内部自引用，并通过 Pin<&mut Self> 锁死内存地址 ===");
    // 使用 std::pin::pin! 将状态机固定在当前栈帧地址上，获得 Pin<&mut SelfRefStateMachine>
    let mut pinned_a = std::pin::pin!(moved_state);
    pinned_a.as_mut().poll_first_step();
    pinned_a.inspect_addresses("pinned_a");

    /*
    // 如果试图从 Pin<&mut SelfRefStateMachine> 中获取 &mut Self 并调用 mem::swap 搬移内存：
    let mut pinned_b = std::pin::pin!(SelfRefStateMachine::new([1, 2, 3, 4]));
    // 编译错误 E0277：`PhantomPinned` cannot be unpinned
    std::mem::swap(pinned_a.get_mut(), pinned_b.get_mut());
    */

    println!("\n=== 3. 工程实战中固定 !Unpin 异步状态机的两种标准方式 ===");
    // 方式 A：使用 std::pin::pin! 零堆分配直接固定在当前栈帧上
    let mut stack_pinned_fut = std::pin::pin!(parse_packet_header());

    // 方式 B：使用 Box::pin 分配并固定在堆内存上（返回 Pin<Box<impl Future>>，可自由传递）
    let mut heap_pinned_fut: Pin<Box<dyn Future<Output = u8>>> = Box::pin(parse_packet_header());

    let mut cx = Context::from_waker(Waker::noop());
    if let Poll::Ready(val) = stack_pinned_fut.as_mut().poll(&mut cx) {
        println!("  [方式 A: std::pin::pin! 栈固定] 轮询结果 = {}", val);
    }
    if let Poll::Ready(val) = heap_pinned_fut.as_mut().poll(&mut cx) {
        println!("  [方式 B: Box::pin 堆固定]       轮询结果 = {}", val);
    }
}
// ANCHOR_END: full
