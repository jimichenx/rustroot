// ANCHOR: full
use std::sync::Arc;
use std::thread;

// ANCHOR: config_def
#[derive(Debug)]
struct AppConfig {
    region: String,
    max_connections: u32,
}
// ANCHOR_END: config_def

fn main() {
    // ANCHOR: arc_demo
    // 1. 将共享配置装入原子引用计数智能指针 Arc<T>
    let config = Arc::new(AppConfig {
        region: String::from("ap-northeast-1"),
        max_connections: 1024,
    });
    println!("初始强引用计数 = {}", Arc::strong_count(&config));

    let mut handles = Vec::new();

    for worker_id in 1..=2 {
        // 2. 在主线程中先调用 Arc::clone：复制 8B 栈指针，并通过硬件原子指令将计数 +1
        let config_for_worker = Arc::clone(&config);

        // 3. 将克隆出的独立 Arc 句柄通过 move 移入后台子线程
        let handle = thread::spawn(move || {
            println!(
                "[后台线程 #{}] 读取配置: region = {}, max_conn = {}",
                worker_id, config_for_worker.region, config_for_worker.max_connections
            );
        }); // 子线程结束时，config_for_worker 触发 Drop，通过硬件原子指令将计数 -1
        handles.push(handle);
    }

    // 主线程自己依然可以随时读取原 config
    println!("[主线程] 自身依然持有配置，当前引用计数 = {}", Arc::strong_count(&config));

    for h in handles {
        h.join().unwrap();
    }

    println!("[主线程] 所有子线程已退场，最终引用计数 = {}", Arc::strong_count(&config));
    // ANCHOR_END: arc_demo
}
// ANCHOR_END: full
