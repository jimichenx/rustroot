// Expect: E0277 (when uncommenting the bad_hold_guard_across_await spawn block)
// ANCHOR: full
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ANCHOR: async_tasks
// 模拟两个耗时的异步网络查询任务：使用 tokio::time::sleep 异步让出线程，绝不阻塞底层 OS 线程
async fn fetch_user_name(user_id: u64) -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    format!("Alice(#{})", user_id)
}

async fn fetch_user_orders(user_id: u64) -> Vec<String> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    vec![format!("订单-{}-A", user_id), format!("订单-{}-B", user_id)]
}
// ANCHOR_END: async_tasks

// ANCHOR: mutex_across_await
// 演示在异步任务中安全使用标准库 std::sync::Mutex：
// 在进入 .await 挂起点之前，用内部代码块 {} 提前释放 MutexGuard
async fn record_and_notify(counter: Arc<Mutex<u64>>, user_id: u64) {
    let current_count = {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        *guard
    }; // <--- guard 在这里离开作用域自动 Drop 解锁，绝不跨过下面的 .await

    // 锁已释放，此时安全地挂起等待异步 I/O，状态机依然满足 Send 契约
    tokio::time::sleep(Duration::from_millis(20)).await;
    println!("  [后台任务 #{}] 已完成通知，当前累计访问计数 = {}", user_id, current_count);
}

/*
// 反面教材：如果拿着标准库的 MutexGuard 跨过 .await，会导致状态机变成 !Send 无法放入 tokio::spawn
async fn bad_hold_guard_across_await(counter: Arc<Mutex<u64>>) {
    let mut guard = counter.lock().unwrap();
    *guard += 1;
    // 错误：guard 被打包进跨 .await 的状态机字段，而 MutexGuard 是 !Send 的
    tokio::time::sleep(Duration::from_millis(20)).await;
    println!("count = {}", *guard);
}
*/
// ANCHOR_END: mutex_across_await

// ANCHOR: tokio_main
#[tokio::main]
async fn main() {
    // 1. 顺序 .await：由于 Future 是惰性的，第二个任务必须等第一个任务完成后才开始构造轮询
    let start_seq = Instant::now();
    let name = fetch_user_name(1).await;
    let orders = fetch_user_orders(1).await;
    println!(
        "[1. 顺序 .await] 拿到 {} 与 {:?}（耗时约 {:?}）",
        name,
        orders,
        start_seq.elapsed()
    );

    // 2. tokio::join!：在当前任务内同时并发轮询两个子状态机（可借用当前栈上局部变量）
    let start_join = Instant::now();
    let local_prefix = String::from("VIP");
    let (name, orders) = tokio::join!(
        async {
            let n = fetch_user_name(2).await;
            format!("{}-{}", local_prefix, n) // 直接借用局部变量 local_prefix
        },
        fetch_user_orders(2)
    );
    println!(
        "[2. tokio::join! 并发] 拿到 {} 与 {:?}（耗时约 {:?}）",
        name,
        orders,
        start_join.elapsed()
    );

    // 3. tokio::spawn：派生独立后台任务交由多线程工作窃取池调度（要求 Send + 'static）
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for id in 1..=3 {
        let counter_cloned = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            record_and_notify(counter_cloned, id).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // 4. tokio::task::spawn_blocking：遇到不可避免的同步阻塞或重度 CPU 计算时，交给专用阻塞池
    let heavy_hash = tokio::task::spawn_blocking(|| {
        // 这里运行在专门的阻塞线程池上，不会卡死异步工作线程
        std::thread::sleep(Duration::from_millis(10));
        "sha256:9f86d081884c7d659a2feaa0c55ad015"
    })
    .await
    .unwrap();
    println!("[4. spawn_blocking] 拿到同步计算结果: {}", heavy_hash);
}
// ANCHOR_END: tokio_main
// ANCHOR_END: full
