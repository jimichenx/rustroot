fn main() {
    // 1. 两个看起来都叫 &i32 的引用，因指向变量的作用域不同，在编译期是不同的类型实体
    let outer_val = 10; // 存活范围记作 'outer
    let r_outer: &i32 = &outer_val; // 真实类型为 &'outer i32

    {
        let inner_val = 20; // 存活范围记作 'inner（比 'outer 短）
        let r_inner: &i32 = &inner_val; // 真实类型为 &'inner i32
        println!("r_outer = {r_outer}, r_inner = {r_inner}");

        // 若试图让外层引用指向内层短命变量，编译器会因范围不满足而拦截（E0597）：
        // r_outer = &inner_val; // ❌ `inner_val` does not live long enough
    }

    // 2. 单函数内的非词法生命周期（NLL）：引用的有效区间只算到「最后一次使用点」
    let mut scores = vec![10, 20, 30];
    let first_view: &i32 = &scores[0];
    println!("first score = {first_view}"); // <-- first_view 的最后一次使用点，借用在此行当场结束

    // 尽管 first_view 变量本身还未离开 {} 作用域，但它的借用区间已经结束，因此这里可以安全修改
    scores.push(40);
    println!("updated scores = {scores:?}");
}
