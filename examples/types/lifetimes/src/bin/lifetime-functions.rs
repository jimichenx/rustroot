// 1. 返回值可能来自 a，也可能来自 b：用同一个泛型参数 'a 声明「返回值与 a、b 都有关系」
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// 2. 返回值只来自 first，与 second 无关：只给 first 和返回值标 'a，不把 second 绑进来
fn pick_first<'a>(first: &'a str, second: &str) -> &'a str {
    println!("ignoring second = {second}");
    first
}

// 3. 只有一个输入引用时，返回值必然来自该输入，编译器自动省略 'a（Elision）
fn first_three_bytes(s: &str) -> &str {
    &s[..3]
}

fn main() {
    let s1 = String::from("RustRoot");
    let result_first: &str;

    {
        let s2 = String::from("hi");
        // longer 返回值与 s1、s2 都挂钩，在 s2 存活的内层块里使用完全合法
        let picked = longer(&s1, &s2);
        println!("longer = {picked}");

        // pick_first 声明返回值只与第一个参数 s1 有关，与短命的 s2 无关
        result_first = pick_first(&s1, &s2);
    } // s2 在这里销毁，但因为 result_first 只关联 s1，所以在外层依然有效

    println!("result_first = {}, prefix = {}", result_first, first_three_bytes(result_first));
}
