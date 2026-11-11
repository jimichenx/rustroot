use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> Self {
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // 返回内部数据的原生只读引用 &T
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0 // 返回内部数据的原生可变引用 &mut T
    }
}

// 得益于标准库为 &T 也实现了 Deref，泛型函数可同时通吃普通引用与各类智能指针
fn read_via_deref<P: Deref<Target = i32>>(ptr: P) -> i32 {
    *ptr
}

fn main() {
    let mut custom = MyBox::new(10);

    // 1. 读取位置：*custom 等价于两步展开 *(Deref::deref(&custom))
    let v1 = *custom;
    let v2 = *(Deref::deref(&custom));
    println!("read: *custom = {v1}, expanded = {v2}");

    // 2. 写入位置：*custom 在赋值号左侧，展开为 *(DerefMut::deref_mut(&mut custom)) = 99
    *custom = 99;
    println!("after write: *custom = {}", *custom);

    // 3. 统一协议协调性：&i32、Box<i32>、MyBox<i32> 均可传入 P: Deref<Target = i32>
    let plain_val = 42;
    println!(
        "generic Deref: &i32 -> {}, Box -> {}, MyBox -> {}",
        read_via_deref(&plain_val),
        read_via_deref(Box::new(77)),
        read_via_deref(custom),
    );
}
