// ANCHOR: full
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    // 1. 单线程演示：认识把数据锁在盒子里的 Mutex<T> 与 MutexGuard
    // ANCHOR: basic_mutex
    // 数据 vec![1, 2] 被直接封装进 Mutex 盒子里，外部不存在裸露的 Vec 变量
    let locked_list = Mutex::new(vec![1, 2]);

    {
        // 调用 .lock() 排队获取钥匙，返回守卫智能指针 MutexGuard<Vec<i32>>
        let mut guard = locked_list.lock().unwrap();
        // 通过 DerefMut 直接向内部的 Vec 追加元素
        guard.push(3);
    } // <- guard 在此处离开作用域触发 Drop，自动释放操作系统互斥锁

    println!("单线程解锁后读取: {:?}", locked_list.lock().unwrap());
    // ANCHOR_END: basic_mutex

    // 2. 多线程实战：用 Arc<Mutex<T>> 让多个线程安全地共同修改同一份数据
    // ANCHOR: arc_mutex
    // 外层 Arc 负责跨线程共享所有权，内层 Mutex 负责排队加锁修改
    let shared_counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter_clone = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // 获取锁守卫 -> 修改内部整数 -> 语句结束守卫自动 Drop 解锁
                *counter_clone.lock().unwrap() += 1;
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    // 4 个线程各加 1000 次，结果精准为 4000，零数据竞争丢失
    println!("4 个线程并发累加结果 = {}", *shared_counter.lock().unwrap());
    // ANCHOR_END: arc_mutex

    // 3. 读多写少场景：读写锁 RwLock<T>
    // ANCHOR: rwlock_demo
    let cache = RwLock::new(String::from("v1.0_config"));

    // 允许多个只读守卫（RwLockReadGuard）同时并存，互不阻塞
    {
        let r1 = cache.read().unwrap();
        let r2 = cache.read().unwrap();
        println!("并发读取缓存: r1 = {}, r2 = {}", *r1, *r2);
    } // r1, r2 离开作用域释放读锁

    // 写入时调用 .write() 获取独占写守卫（RwLockWriteGuard）
    {
        let mut w = cache.write().unwrap();
        w.push_str("_updated");
    }
    println!("更新后缓存: {}", *cache.read().unwrap());
    // ANCHOR_END: rwlock_demo
}
// ANCHOR_END: full
