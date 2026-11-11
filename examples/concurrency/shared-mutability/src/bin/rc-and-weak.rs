// ANCHOR: full
use std::rc::{Rc, Weak};

// ANCHOR: list_node
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<Node>>,
}
// ANCHOR_END: list_node

// ANCHOR: tree_weak
#[derive(Debug)]
struct BiNode {
    val: i32,
    // 后继节点只用 Weak 指回前驱节点，不增加强引用计数，不阻止前驱节点释放
    prev: Option<Weak<BiNode>>,
}
// ANCHOR_END: tree_weak

fn main() {
    // 1. 演示两个链表分支节点通过 Rc 共享同一个后续节点
    // ANCHOR: rc_share
    let shared_node = Rc::new(Node {
        val: 10,
        next: None,
    });
    println!("创建 shared_node 后，强引用计数 = {}", Rc::strong_count(&shared_node));

    // node_a.next 指向 shared_node：Rc::clone 只复制 8B 指针并将计数 +1
    let node_a = Node {
        val: 3,
        next: Some(Rc::clone(&shared_node)),
    };
    println!("node_a 接入后，强引用计数 = {}", Rc::strong_count(&shared_node));

    {
        // node_b.next 同样指向 shared_node
        let node_b = Node {
            val: 4,
            next: Some(Rc::clone(&shared_node)),
        };
        println!("node_b 接入后，强引用计数 = {}", Rc::strong_count(&shared_node));
        println!("通过 node_b 读取后续节点值: {}", node_b.next.as_ref().unwrap().val);
    } // node_b 离开作用域，触发 Rc 的 Drop，强引用计数自动 -1

    println!("node_b 退场后，强引用计数 = {}", Rc::strong_count(&shared_node));
    println!(
        "node_a (val={}) 的后续节点依然完好: val={}",
        node_a.val,
        node_a.next.as_ref().unwrap().val
    );
    // ANCHOR_END: rc_share

    // 2. 演示链表节点用 Weak 指回前驱节点与 upgrade() 安全探测
    // ANCHOR: weak_demo
    let second_node: BiNode;

    {
        let first_node = Rc::new(BiNode {
            val: 1,
            prev: None,
        });

        // downgrade 产生 Weak<BiNode>：强计数保持 1，弱计数变为 1
        second_node = BiNode {
            val: 2,
            prev: Some(Rc::downgrade(&first_node)),
        };

        println!(
            "作用域内: strong = {}, weak = {}",
            Rc::strong_count(&first_node),
            Rc::weak_count(&first_node)
        );

        // 读取 Weak 必须先调用 upgrade()，返回 Option<Rc<BiNode>>
        if let Some(prev_rc) = second_node.prev.as_ref().unwrap().upgrade() {
            println!("节点 {} 成功找到前驱节点: val = {}", second_node.val, prev_rc.val);
        }
    } // first_node 离开作用域，强计数归 0，前驱节点当场释放

    // 此时再去 upgrade，安全返回 None，无悬垂指针
    println!(
        "first_node 退场后，节点 {} 尝试 upgrade 前驱结果: {:?}",
        second_node.val,
        second_node.prev.as_ref().unwrap().upgrade()
    );
    // ANCHOR_END: weak_demo
}
// ANCHOR_END: full
