// ANCHOR: full
use std::marker::PhantomData;
use std::cell::RefCell;
use std::thread::{self, JoinHandle};

// ANCHOR: mini_scope_def
// 极简教学版 Scope：携带两个生命周期参数
// 'scope: 本次 mini_scope 函数调用自身的执行区间
// 'env:   被子线程借用的外部局部变量（如 data）的存活区间，要求 'env 至少包住 'scope
pub struct MiniScope<'scope, 'env: 'scope> {
    handles: RefCell<Vec<JoinHandle<()>>>,
    // PhantomData 用来向编译器声明 'scope 与 'env 的型变约束关系，运行时占 0 字节
    _marker: PhantomData<&'scope &'env ()>,
}

impl<'scope, 'env> MiniScope<'scope, 'env> {
    // 关键签名：把普通 thread::spawn 的 F: 'static 放宽为 F: 'env
    pub fn spawn<F>(&'scope self, f: F)
    where
        F: FnOnce() + Send + 'env,
    {
        // 将只活 'env 的闭包转为裸指针再封装为 'static 传给底层 OS 线程
        // SAFETY: 下方的 mini_scope 函数在返回前必定 join 所有子线程，
        // 保证子线程绝不可能活过 'scope，更不可能活过 'env。
        let boxed: Box<dyn FnOnce() + Send + 'env> = Box::new(f);
        let static_box: Box<dyn FnOnce() + Send + 'static> = unsafe {
            std::mem::transmute(boxed)
        };
        let handle = thread::spawn(static_box);
        self.handles.borrow_mut().push(handle);
    }
}

pub fn mini_scope<'env, F>(body: F)
where
    F: for<'scope> FnOnce(&'scope MiniScope<'scope, 'env>),
{
    let scope = MiniScope {
        handles: RefCell::new(Vec::new()),
        _marker: PhantomData,
    };
    // 1. 执行用户传入的闭包，在内部派发子线程
    body(&scope);

    // 2. 焊死在出口的护栏：在 mini_scope 返回前，强制阻塞等待所有子线程结束
    for handle in scope.handles.borrow_mut().drain(..) {
        handle.join().unwrap();
    }
}
// ANCHOR_END: mini_scope_def

fn main() {
    // ANCHOR: mini_scope_demo
    let local_nums = vec![10, 20, 30, 40];

    // 使用我们手写的 mini_scope，直接借用栈上的 local_nums
    mini_scope(|s| {
        s.spawn(|| {
            let sum: i32 = local_nums.iter().sum();
            println!("[手写 MiniScope 子线程] 直接借用局部变量求和 = {}", sum);
        });
    });

    println!("[主线程] mini_scope 已安全汇合，local_nums 依然完好: {:?}", local_nums);
    // ANCHOR_END: mini_scope_demo
}
// ANCHOR_END: full
