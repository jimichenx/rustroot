use std::fmt;

// 字段中含有引用时，结构体必须声明生命周期泛型参数 <'a>
struct KvView<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> KvView<'a> {
    // 接收 &str 并返回 KvView<'_>：利用匿名生命周期 '_ 自动将返回视图关联到输入 line
    fn parse(line: &str) -> KvView<'_> {
        let (k, v) = line.split_once('=').unwrap_or((line, ""));
        KvView { key: k, value: v }
    }

    // 命中省略规则 3（&self），返回值自动与 self 关联，无需手写 'a
    fn key_part(&self) -> &str {
        self.key
    }
}

// 为结构体实现 Trait 时，同样可用 '_ 省略显式声明 impl<'a>
impl fmt::Display for KvView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} => {}]", self.key, self.value)
    }
}

fn main() {
    let config_line = String::from("domain=rustroot.cn");
    let view = KvView::parse(&config_line);
    println!("parsed view: {view}, key = {}", view.key_part());

    // 依附关系约束：KvView 实例绝不能比它借用的源数据活得更久
    // let dangling_view: KvView<'_>;
    // {
    //     let temp = String::from("port=8080");
    //     dangling_view = KvView::parse(&temp); // ❌ E0597: `temp` does not live long enough
    // }
}
