// ANCHOR: full
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Wake, Waker};

// ANCHOR: custom_future
// 手写一个实现标准库 Future 契约的状态机：模拟一次异步网络查询
struct QueryUserFuture {
    username: String,
    io_completed: bool,
}

impl QueryUserFuture {
    fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
            io_completed: false,
        }
    }
}

impl Future for QueryUserFuture {
    type Output = u64;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 普通结构体不含自引用，可通过 get_mut() 直接取得 &mut Self 修改字段
        let this = self.get_mut();

        if !this.io_completed {
            println!(
                "  [QueryUserFuture::poll] 第一次被轮询：向网卡发起查询 `{}`，数据尚未返回",
                this.username
            );
            this.io_completed = true;

            // 关键一步：在返回 Pending 让出线程前，拿到当前任务的 Waker 句柄。
            // 真实运行时会把它注册给操作系统 epoll/kqueue；这里直接触发模拟「网卡数据随后到达」
            let waker: Waker = cx.waker().clone();
            println!("  [Event Loop] 网卡数据到达，调用 waker.wake() 通知运行时重新调度该任务");
            waker.wake();

            Poll::Pending
        } else {
            println!("  [QueryUserFuture::poll] 第二次被轮询：网卡数据已就绪，交出 user_id = 42");
            Poll::Ready(42)
        }
    }
}
// ANCHOR_END: custom_future

// ANCHOR: async_fn_caller
// 编译器会自动将 async fn 展开为外层 Future 状态机，并在 .await 处调用子任务的 poll
async fn fetch_user_profile(username: &str) -> String {
    println!("[fetch_user_profile] 开始执行，准备等待子任务 QueryUserFuture...");
    let user_id = QueryUserFuture::new(username).await;
    println!("[fetch_user_profile] 跨越 .await 恢复执行，拿到 user_id = {}", user_id);
    format!("用户 {} (ID: #{})", username, user_id)
}
// ANCHOR_END: async_fn_caller

// ANCHOR: mini_executor
// 实现标准库 Wake 契约：当 waker.wake() 被调用时，将就绪标记设为 true
struct TaskReadySignal {
    is_ready: AtomicBool,
}

impl Wake for TaskReadySignal {
    fn wake(self: Arc<Self>) {
        self.is_ready.store(true, Ordering::SeqCst);
    }
}

fn main() {
    println!("1. 调用 async fn 构造状态机（注意：此时函数体内部一行代码都不会执行）");
    let future = fetch_user_profile("alice");
    println!("2. 状态机已构造完毕，准备交给执行器首次 poll\n");

    // 将状态机固定在栈上（Pin 的底层原因将在 14.4 节专门拆解）
    let mut pinned_future = std::pin::pin!(future);

    // 构造与该任务绑定的 Waker 唤醒句柄（初始状态为就绪，允许执行第一次 poll）
    let signal = Arc::new(TaskReadySignal {
        is_ready: AtomicBool::new(true),
    });
    let waker = Waker::from(Arc::clone(&signal));
    let mut cx = Context::from_waker(&waker);

    let mut poll_round = 0;
    // 只有当任务处于 is_ready 就绪状态时，工作线程才会去 poll 它，绝不盲目空转
    while signal.is_ready.swap(false, Ordering::SeqCst) {
        poll_round += 1;
        println!("--- 执行器发起第 {} 次 poll ---", poll_round);
        match pinned_future.as_mut().poll(&mut cx) {
            Poll::Pending => {
                println!("--- 第 {} 次 poll 返回 Pending：任务已让出线程 ---\n", poll_round);
            }
            Poll::Ready(profile) => {
                println!("--- 第 {} 次 poll 返回 Ready：拿到最终结果 `{}` ---", poll_round, profile);
                break;
            }
        }
    }
}
// ANCHOR_END: mini_executor
// ANCHOR_END: full
