use std::fmt::Debug;

fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// 泛型约束 T: 'static：要求类型 T 自身不依附于任何短暂的局部借用
fn accept_owned_or_static<T: 'static + Debug>(item: T) {
    println!("accepted T: 'static -> {item:?}");
} // item 在此离开作用域并立即释放（证明满足 T: 'static 的值并不需要活到程序结束）

fn main() {
    // 1. 范围包含与归属（协变）：'static 与外层范围均可自动缩减归属为内层小范围
    let global_lit: &'static str = "ALWAYS_ALIVE";
    let outer_string = String::from("outer_scope");
    {
        let inner_string = String::from("in");
        // global_lit ('static) 与 &inner_string (内层范围) 对齐到共同的内层小范围
        let res1 = longer(global_lit, &inner_string);
        let res2 = longer(&outer_string, &inner_string);
        println!("converged in inner scope: res1 = {res1}, res2 = {res2}");
    }

    // 2. 区分 &'static str 与 T: 'static
    accept_owned_or_static(global_lit); // &'static str 满足 T: 'static

    let local_string = String::from("short-lived local String");
    // 局部 String 虽然马上会被销毁，但它不借用任何外部局部变量，100% 满足 T: 'static
    accept_owned_or_static(local_string);

    // 反例：指向局部变量的引用携带短暂范围，不满足 T: 'static
    // let another_local = String::from("temp");
    // accept_owned_or_static(&another_local); // ❌ E0597: `another_local` does not live long enough
}
