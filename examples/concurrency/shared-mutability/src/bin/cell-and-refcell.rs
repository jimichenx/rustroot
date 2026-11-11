// ANCHOR: full
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// ANCHOR: cell_node
#[derive(Debug)]
struct Node {
    // 用 Cell 包裹标量字段：不对外借出 &i32，只允许整进整出
    val: Cell<i32>,
    next: Option<Rc<Node>>,
}
// ANCHOR_END: cell_node

// ANCHOR: refcell_tracker
trait Messenger {
    // 外部接口签名锁死了只读接收者 &self
    fn send(&self, msg: &str);
}

struct MockQueue {
    // 用 RefCell 包裹动态数组：将读写排他检查推迟到运行时
    sent_messages: RefCell<Vec<String>>,
}

impl Messenger for MockQueue {
    fn send(&self, msg: &str) {
        // borrow_mut() 在运行时将 borrow 计数置为 -1，并返回 RefMut<Vec<String>> 守卫
        self.sent_messages.borrow_mut().push(String::from(msg));
    } // 函数结束，RefMut 守卫触发 Drop，自动将 borrow 计数恢复为 0
}
// ANCHOR_END: refcell_tracker

fn main() {
    // 1. 演示通过 Cell<i32> 在共享只读的 Rc<Node> 下安全修改字段值
    // ANCHOR: cell_demo
    let shared_node = Rc::new(Node {
        val: Cell::new(10),
        next: None,
    });
    let node_a = Node {
        val: Cell::new(3),
        next: Some(Rc::clone(&shared_node)),
    };
    let node_b = Node {
        val: Cell::new(4),
        next: Some(Rc::clone(&shared_node)),
    };

    // 尽管 shared_node 没有声明 mut 且被多方共享，仍可通过 .set() 整体替换内部的 i32
    shared_node.val.set(99);

    println!(
        "修改后：node_a 看到的后续值 = {}, node_b 看到的后续值 = {}",
        node_a.next.as_ref().unwrap().val.get(),
        node_b.next.as_ref().unwrap().val.get()
    );
    // ANCHOR_END: cell_demo

    // 2. 演示 RefCell<Vec<String>> 与守卫 Ref / RefMut 的作用域生命周期
    // ANCHOR: refcell_demo
    let queue = MockQueue {
        sent_messages: RefCell::new(Vec::new()),
    };

    // 通过只读的 &queue 调用 send，内部成功向 Vec 追加元素
    queue.send("handshake_init");
    queue.send("heartbeat_ok");

    {
        // borrow() 将运行时计数 +1，返回只读守卫 Ref<Vec<String>>
        let reader_1 = queue.sent_messages.borrow();
        let reader_2 = queue.sent_messages.borrow();
        println!("两个只读守卫并存，消息总数 = {}, 首条 = {}", reader_1.len(), reader_2[0]);

        // ⚠️ 如果在此处 reader_1 和 reader_2 尚未退场时调用 queue.send("boom")，
        // 内部执行 borrow_mut() 就会因计数为 2（非 0）而在运行时当场触发 panic:
        // thread 'main' panicked at 'already borrowed: BorrowMutError'
    } // reader_1 和 reader_2 离开作用域触发 Drop，计数减回 0

    // 只读守卫全部退场后，再次申请可变借用完全安全
    queue.send("session_closed");
    println!("最终消息列表: {:?}", queue.sent_messages.borrow());
    // ANCHOR_END: refcell_demo
}
// ANCHOR_END: full
