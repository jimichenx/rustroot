fn add_ten(target: &mut i32) {
    *target += 10;
}

struct Editor {
    buffer: String,
    cursor: usize,
}

impl Editor {
    // 若写成 fn log_cursor(&self)，签名会锁死整个 Editor，导致持有 &mut self.buffer 时无法调用
    // 收敛参数粒度：只接收真正需要的 cursor 字段，即可与 &mut self.buffer 和平共处
    fn log_cursor(cursor: usize) {
        println!("cursor at {cursor}");
    }
}

// 用与内存地址无关的下标区间 (usize, usize) 替代自引用指针，结构体可自由移动（Move）
struct IndexedToken {
    raw: String,
    key_span: (usize, usize),
}

impl IndexedToken {
    fn new(raw: String, key_span: (usize, usize)) -> Self {
        IndexedToken { raw, key_span }
    }

    fn key(&self) -> &str {
        &self.raw[self.key_span.0..self.key_span.1]
    }
}

fn main() {
    // 1. 可变引用的重借用（Reborrowing）：add_ten(r1) 隐式展开为 add_ten(&mut *r1)
    let mut val = 20;
    let r1: &mut i32 = &mut val;
    add_ten(r1); // 临时借出短范围可变引用，调用结束后 r1 立即恢复可用
    *r1 += 5;
    println!("after reborrow: *r1 = {r1}");

    // 2. 结构体字段借用拆分（Split Borrowing）
    let mut editor = Editor {
        buffer: String::from("fn main() "),
        cursor: 10,
    };
    let buf_mut: &mut String = &mut editor.buffer;
    Editor::log_cursor(editor.cursor); // 仅访问 cursor 字段，不与 buf_mut 冲突
    buf_mut.push_str("{}");
    println!("editor buffer = {}", editor.buffer);

    // 3. 下标偏移量破局自引用困局：结构体发生 Move 搬家后依然 100% 安全
    let token = IndexedToken::new(String::from("host=rustroot.cn"), (0, 4));
    let moved_token = token; // 按值移动结构体，毫无悬垂风险
    println!("moved_token key = {}", moved_token.key());
}
