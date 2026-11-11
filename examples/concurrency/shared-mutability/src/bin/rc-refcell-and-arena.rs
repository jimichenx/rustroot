// ANCHOR: full
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// ANCHOR: rc_refcell_node
type NodeHandle = Rc<RefCell<BiNode>>;

#[derive(Debug)]
struct BiNode {
    val: i32,
    // 向后持有所有权（强引用），向前僅观察（弱引用）
    next: Option<NodeHandle>,
    prev: Option<Weak<RefCell<BiNode>>>,
}
// ANCHOR_END: rc_refcell_node

// ANCHOR: arena_graph
#[derive(Debug)]
struct ArenaNode {
    val: i32,
    // 不存任何指针或 Rc，只存目标节点在 Vec 中的下标索引
    prev: Option<usize>,
    next: Option<usize>,
}

struct ListArena {
    nodes: Vec<ArenaNode>,
}

impl ListArena {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn alloc(&mut self, val: i32) -> usize {
        let id = self.nodes.len();
        self.nodes.push(ArenaNode {
            val,
            prev: None,
            next: None,
        });
        id
    }

    fn link(&mut self, first_id: usize, second_id: usize) {
        self.nodes[first_id].next = Some(second_id);
        self.nodes[second_id].prev = Some(first_id);
    }
}
// ANCHOR_END: arena_graph

fn main() {
    // 1. 演示 Rc<RefCell<BiNode>> 动态连接双向链表并修改节点
    // ANCHOR: rc_refcell_demo
    let first = Rc::new(RefCell::new(BiNode {
        val: 10,
        next: None,
        prev: None,
    }));

    let second = Rc::new(RefCell::new(BiNode {
        val: 20,
        next: None,
        prev: Some(Rc::downgrade(&first)),
    }));

    // 通过 borrow_mut() 取得可变守卫，回头把 first.next 指向 second
    first.borrow_mut().next = Some(Rc::clone(&second));

    // 通过 first 顺着 next 修改 second 的值
    if let Some(next_rc) = &first.borrow().next {
        next_rc.borrow_mut().val += 5;
    }

    println!(
        "Rc<RefCell> 双向验证: first.val = {}, first->next.val = {}, second->prev.val = {}",
        first.borrow().val,
        first.borrow().next.as_ref().unwrap().borrow().val,
        second.borrow().prev.as_ref().unwrap().upgrade().unwrap().borrow().val
    );
    // ANCHOR_END: rc_refcell_demo

    // 2. 演示基于 Vec + usize 的 Arena 索引双向链表
    // ANCHOR: arena_demo
    let mut arena = ListArena::new();
    let n1 = arena.alloc(10);
    let n2 = arena.alloc(20);

    // 直接通过下标连接与修改，100% 走编译期静态借用检查
    arena.link(n1, n2);
    if let Some(next_id) = arena.nodes[n1].next {
        arena.nodes[next_id].val += 5;
    }

    let next_id = arena.nodes[n1].next.unwrap();
    let prev_id = arena.nodes[n2].prev.unwrap();
    println!(
        "Arena 索引双向验证: n1.val = {}, n1->next.val = {}, n2->prev.val = {}",
        arena.nodes[n1].val, arena.nodes[next_id].val, arena.nodes[prev_id].val
    );
    // ANCHOR_END: arena_demo
}
// ANCHOR_END: full
