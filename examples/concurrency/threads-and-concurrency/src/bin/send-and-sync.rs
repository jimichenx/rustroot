// ANCHOR: full
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

// ANCHOR: auto_trait_struct
// 我们从未手动写过 impl Send 或 impl Sync，
// 但因为 String 和 u32 都同时具备 Send + Sync 标签，
// 编译器会自动为 AppReport 推导出 Send + Sync
#[derive(Debug)]
struct AppReport {
    title: String,
    code: u32,
}
// ANCHOR_END: auto_trait_struct

fn main() {
    // 1. 验证普通结构体自动具备 Send（可移交所有权）与 Sync（可跨线程共享 &T）
    // ANCHOR: send_and_sync_demo
    let report = AppReport {
        title: String::from("daily_health"),
        code: 200,
    };

    // 动作一（Sync）：通过 thread::scope 让两个子线程同时拿着 &AppReport 共享读取
    thread::scope(|s| {
        s.spawn(|| println!("[Sync 共享读 A] title = {}", report.title));
        s.spawn(|| println!("[Sync 共享读 B] code = {}", report.code));
    });

    // 动作二（Send）：通过 move 将 report 的唯一所有权打包发送给另一个线程独占消费
    let handle = thread::spawn(move || {
        println!("[Send 独占移交] 子线程接管并销毁 report: {:?}", report);
    });
    handle.join().unwrap();
    // ANCHOR_END: send_and_sync_demo

    // 2. 验证第二象限：RefCell<T> 是 Send（可独占搬家），但绝不是 Sync（不可多线程共享 &T）
    // ANCHOR: refcell_send_not_sync
    let solo_cell = RefCell::new(vec![1, 2]);

    // ✅ 合法：RefCell 满足 Send，可以把唯一所有权打包 move 给单个子线程独占使用
    let cell_worker = thread::spawn(move || {
        solo_cell.borrow_mut().push(3);
        println!("[RefCell 独占搬家] 子线程内部安全修改: {:?}", solo_cell.borrow());
    });
    cell_worker.join().unwrap();

    // ❌ 若试图通过 Arc<RefCell<T>> 让多个线程共享 &RefCell，编译器当场拦截 E0277：
    //    `RefCell<Vec<i32>>` cannot be shared between threads safely (the trait `Sync` is not implemented)
    // ✅ 换成实现了 Sync 的 Mutex<T>，Arc<Mutex<T>> 即可安全跨线程共享：
    let shared_safe = Arc::new(Mutex::new(vec![10, 20]));
    let cloned = Arc::clone(&shared_safe);
    thread::spawn(move || {
        cloned.lock().unwrap().push(30);
    })
    .join()
    .unwrap();
    println!("[Arc<Mutex<T>> 组合] 跨线程修改结果: {:?}", shared_safe.lock().unwrap());
    // ANCHOR_END: refcell_send_not_sync
}
// ANCHOR_END: full
