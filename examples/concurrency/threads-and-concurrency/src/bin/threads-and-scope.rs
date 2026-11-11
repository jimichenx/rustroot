// ANCHOR: full
use std::thread;

fn main() {
    // 1. 最基础的线程创建与 join 等待返回值
    // ANCHOR: basic_spawn
    // 启动一个操作系统子线程，传入无参闭包
    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=5 {
            sum += i;
        }
        println!("[子线程] 计算完成，准备返回结果");
        sum // 闭包的返回值会由 JoinHandle 接住
    });

    println!("[主线程] 已派发子线程，正在等待其汇合...");
    // 调用 .join() 阻塞等待子线程执行完毕，并取出返回值 15
    let result = handle.join().unwrap();
    println!("[主线程] 收到子线程计算结果: {}", result);
    // ANCHOR_END: basic_spawn

    // 2. 通过 move 闭包将主线程数据的唯一所有权打包移交给子线程
    // ANCHOR: move_spawn
    let task_batch = vec![10, 20, 30];

    // ❌ 如果不加 move：闭包默认只借用 &task_batch，编译器报错 E0373
    // （因为子线程可能活得比当前 main 栈帧更长，导致 &task_batch 悬垂）
    // ✅ 加上 move：将 task_batch 的唯一所有权整体搬入子线程自己的栈帧
    let worker = thread::spawn(move || {
        println!("[move 子线程] 独占接手批次数据: {:?}", task_batch);
        task_batch.iter().sum::<i32>()
    });

    // 注意：此时主线程里的 task_batch 已因 Move 失效，主线程不能再碰它
    let batch_sum = worker.join().unwrap();
    println!("[主线程] 批次汇总 = {}", batch_sum);
    // ANCHOR_END: move_spawn

    // 3. 当父线程明确等待子线程在当前块内跑完时，用 thread::scope 直接零成本借用
    // ANCHOR: scoped_threads
    let shared_logs = vec![
        String::from("INFO: server started"),
        String::from("WARN: high memory usage"),
        String::from("INFO: request handled"),
    ];

    // thread::scope 承诺在闭包结束前自动 join 内部创建的所有子线程
    thread::scope(|s| {
        // 子线程 1 直接只读借用主线程栈上的 &shared_logs，无需 move 或克隆
        s.spawn(|| {
            println!("[Scope 子线程 A] 日志总条数 = {}", shared_logs.len());
        });

        // 子线程 2 同时只读借用同一份 &shared_logs
        s.spawn(|| {
            let warn_count = shared_logs.iter().filter(|l| l.starts_with("WARN")).count();
            println!("[Scope 子线程 B] 告警条数 = {}", warn_count);
        });
    }); // <- 离开此大括号前，两个子线程保证已全部执行完毕退场

    // 走出 scope 后，主线程自己的 shared_logs 依然完好可用
    println!("[主线程] Scope 结束后继续读取首条日志: {}", shared_logs[0]);
    // ANCHOR_END: scoped_threads
}
// ANCHOR_END: full
