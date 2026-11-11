struct TrackedPart(&'static str);

impl Drop for TrackedPart {
    fn drop(&mut self) {
        println!("  [Drop Part] {}", self.0);
    }
}

struct Session {
    id: u32,
    first_field: TrackedPart,
    second_field: TrackedPart,
}

impl Session {
    fn describe(&self) {
        println!(
            "Session #{} holds ({}, {})",
            self.id, self.first_field.0, self.second_field.0
        );
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        println!("[Drop Session #{}] custom cleanup before fields", self.id);
    }
}

// RAII 资源守卫示例：进门获取锁状态，离开作用域（无论正常结束或提前 return）自动归还
struct LockGuard<'a> {
    resource_name: &'a str,
}

impl<'a> LockGuard<'a> {
    fn acquire(resource_name: &'a str) -> Self {
        println!("[LockGuard] acquired lock on '{resource_name}'");
        LockGuard { resource_name }
    }
}

impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        println!("[LockGuard] auto-released lock on '{}'", self.resource_name);
    }
}

fn process_request(early_exit: bool) {
    let _guard = LockGuard::acquire("db_conn_pool");
    println!("  processing step 1...");
    if early_exit {
        println!("  early return triggered");
        return; // 即使在此处提前返回，_guard 离开作用域依然 100% 触发 Drop
    }
    println!("  processing step 2...");
}

fn main() {
    println!("=== 1. Early drop & reverse drop order ===");
    {
        let s1 = Session {
            id: 1,
            first_field: TrackedPart("s1.first_field"),
            second_field: TrackedPart("s1.second_field"),
        };
        let s2 = Session {
            id: 2,
            first_field: TrackedPart("s2.first_field"),
            second_field: TrackedPart("s2.second_field"),
        };
        let s3 = Session {
            id: 3,
            first_field: TrackedPart("s3.first_field"),
            second_field: TrackedPart("s3.second_field"),
        };
        s1.describe();
        s2.describe();
        s3.describe();

        // s1.drop(); // ❌ 编译错误 E0040: explicit use of destructor method
        println!("calling drop(s1) manually before scope ends:");
        drop(s1); // ✅ 把 s1 的所有权移入空函数 pub fn drop<T>(_x: T) {}，当场销毁
        println!("after drop(s1), s2 and s3 are still alive");
    } // 离开作用域时，存活的局部变量按声明逆序释放：先 s3，后 s2

    println!("\n=== 2. RAII Guard on early return ===");
    process_request(true);
}
