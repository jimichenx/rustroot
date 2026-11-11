use std::path::Path;

fn greet(name: &str) {
    println!("Hello, {name}");
}

fn sum_slice(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// 对比：Path 与 String 之间没有 Deref 父子关系，使用 AsRef<Path> 做显式轻量视图转换
fn print_extension<P: AsRef<Path>>(path: P) {
    let p: &Path = path.as_ref();
    println!("extension = {:?}", p.extension());
}

fn main() {
    // 1. 函数实参触发多层 Deref Coercion：&Box<String> -> &String -> &str
    let boxed_name: Box<String> = Box::new(String::from("RustRoot"));
    greet(&boxed_name);

    // 2. 显式类型标注的 let 绑定触发 Deref Coercion，且支持 &mut T -> &U 只读降级
    let mut numbers: Vec<i32> = vec![10, 20, 30];
    let slice_view: &[i32] = &mut numbers;
    println!("sum_slice = {}", sum_slice(slice_view));

    // 3. 方法调用点自动解引用（Method Autoderef）：
    // Box<String> 自身没有 is_ascii()，顺着 Deref 链找到 str::is_ascii(&self)
    println!("is_ascii = {}", boxed_name.is_ascii());

    // 4. AsRef<Path> 通用参数接口：可同时传入 &str、String 或 PathBuf
    print_extension("config/app.toml");
    print_extension(String::from("src/main.rs"));
}
