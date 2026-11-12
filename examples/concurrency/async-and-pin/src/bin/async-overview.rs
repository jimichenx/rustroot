// ANCHOR: full
// 模拟一段包含两步异步网络请求（先查用户 ID，再查订单列表）的业务流程，
// 直观对比「编译器展开后的状态机 enum」与「程序员手写的 async / await」

// ANCHOR: manual_state_machine
// 编译器在幕后自动为 async fn 生成的状态机枚举示意：
// 跨越挂起点的局部变量（如 user_id）不再住在线程栈上，而是直接变成枚举变体的字段
enum OrderFlowStateMachine {
    // 阶段 0：初始状态，准备发起第一步请求
    Start { username: String },
    // 阶段 1：正在等待第一步「查用户 ID」返回
    WaitingUserId,
    // 阶段 2：已拿到 user_id，正在等待第二步「查订单列表」返回
    WaitingOrders { user_id: u64 },
    // 阶段 3：全部完成
    Done,
}

impl OrderFlowStateMachine {
    // 模拟背后那台「永动机（Event Loop）」每次调用推进状态机一步
    fn step(&mut self) -> Option<String> {
        match self {
            OrderFlowStateMachine::Start { username } => {
                println!("[状态机阶段 0] 发起查用户请求: {} -> 让出线程", username);
                *self = OrderFlowStateMachine::WaitingUserId;
                None // 尚未完成，主动退出线程占用
            }
            OrderFlowStateMachine::WaitingUserId => {
                let resolved_id = 42;
                println!("[状态机阶段 1] 收到 user_id = {}，发起查订单请求 -> 让出线程", resolved_id);
                *self = OrderFlowStateMachine::WaitingOrders { user_id: resolved_id };
                None // 再次让出线程占用
            }
            OrderFlowStateMachine::WaitingOrders { user_id } => {
                let result = format!("用户 #{} 的 3 笔历史订单", user_id);
                println!("[状态机阶段 2] 订单数据就绪，任务全部完成");
                *self = OrderFlowStateMachine::Done;
                Some(result)
            }
            OrderFlowStateMachine::Done => None,
        }
    }
}
// ANCHOR_END: manual_state_machine

fn main() {
    // ANCHOR: drive_loop
    let mut task = OrderFlowStateMachine::Start {
        username: String::from("alice"),
    };

    println!("=== 模拟异步运行时（Event Loop 永动机）驱动状态机 ===");
    // 永动机循环：每次调用 step() 推进一步，中间任务主动让出线程时线程可去干别的活
    loop {
        if let Some(final_orders) = task.step() {
            println!("=== 拿到最终结果: {} ===", final_orders);
            break;
        }
        println!("  (当前任务等待 I/O 中，工作线程转身去处理其他成千上万个连接...)");
    }
    // ANCHOR_END: drive_loop
}
// ANCHOR_END: full
