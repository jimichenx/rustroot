// ANCHOR: full
use std::sync::mpsc;
use std::thread;

// ANCHOR: log_entry
#[derive(Debug)]
struct LogReport {
    worker_id: u32,
    payload: String,
}
// ANCHOR_END: log_entry

fn main() {
    // 1. 基础单向通信与所有权跨线程移交
    // ANCHOR: basic_channel
    // 创建通道，返回成对的 (发送端 tx, 接收端 rx)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let report = LogReport {
            worker_id: 1,
            payload: String::from("chunk_01_parsed"),
        };

        // send(val) 按值接收参数，将 report 的唯一所有权跨线程转移给接收端 rx
        tx.send(report).unwrap();

        // ❌ 编译报错 E0382: borrow of moved value: `report`
        // 发送之后如果子线程还试图读取或修改 report，编译器当场拦截：
        // println!("{:?}", report);
    });

    // 主线程调用 recv() 阻塞等待，接手从子线程移交过来的 LogReport 所有权
    let received = rx.recv().unwrap();
    println!(
        "[主线程] 收到并接管数据: worker #{} -> {}",
        received.worker_id, received.payload
    );
    // ANCHOR_END: basic_channel

    // 2. 多生产者汇聚（克隆多个 tx）与 for 循环持续接收
    // ANCHOR: multi_producer
    let (tx, rx) = mpsc::channel();

    for id in 1..=3 {
        // 为每个工作线程克隆一个独立的发送端把手 tx_clone
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let msg = format!("worker #{} 阶段性产出", id);
            tx_clone.send(msg).unwrap();
        }); // 子线程结束时，其持有的 tx_clone 自动触发 Drop 销毁
    }

    // ⚠️ 关键工程细节：主线程自己手里还攥着最初的那个原版 tx，
    // 必须显式 drop(tx) 把它关掉，否则下方的 for 循环会以为还有人要发消息而永远卡住
    drop(tx);

    // 直接将 rx 当作迭代器遍历，直到所有发送端全部销毁后自动退出循环
    for message in rx {
        println!("[汇总线程] 收到流水线消息: {}", message);
    }
    println!("[汇总线程] 所有发送端已关闭，流水线正常结束");
    // ANCHOR_END: multi_producer
}
// ANCHOR_END: full
