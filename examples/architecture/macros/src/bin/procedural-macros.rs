// ANCHOR: full
use std::fmt;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

// ANCHOR: derive_desugar
// 1. Using compiler built-in #[derive(Debug, PartialEq)]
#[derive(Debug, PartialEq)]
struct DerivedPoint {
    x: i32,
    y: i32,
}

// 2. What #[derive(Debug, PartialEq)] actually appends beneath the struct
struct ExpandedPoint {
    x: i32,
    y: i32,
}

impl fmt::Debug for ExpandedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExpandedPoint")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl PartialEq for ExpandedPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
// ANCHOR_END: derive_desugar

// ANCHOR: attr_desugar
// Minimal block_on runner to illustrate what #[tokio::main] wraps around an async body
fn mini_block_on<F: Future>(fut: F) -> F::Output {
    let mut pinned = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    match pinned.as_mut().poll(&mut cx) {
        Poll::Ready(val) => val,
        Poll::Pending => panic!("demo future should complete immediately"),
    }
}

async fn fetch_port() -> u16 {
    8080
}

// Equivalent synchronous entry function generated after an attribute macro rewrites `async fn`
fn expanded_async_entry() -> u16 {
    mini_block_on(async {
        let port = fetch_port().await;
        port + 1
    })
}
// ANCHOR_END: attr_desugar

fn main() {
    let p1 = DerivedPoint { x: 3, y: 4 };
    let p2 = ExpandedPoint { x: 3, y: 4 };
    println!("[#[derive] 自动生成] {p1:?} | 相等核验 = {}", p1 == DerivedPoint { x: 3, y: 4 });
    println!("[手动还原展开结果]   {p2:?} | 相等核验 = {}", p2 == ExpandedPoint { x: 3, y: 4 });

    let bound_port = expanded_async_entry();
    println!("[属性宏包装还原] 同步入口驱动 async 块返回端口 = {bound_port}");
}
// ANCHOR_END: full
