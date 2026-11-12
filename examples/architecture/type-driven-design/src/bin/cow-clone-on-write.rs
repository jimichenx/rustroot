// ANCHOR: full
use std::borrow::Cow;

// ANCHOR: sanitize_cow
// 写时复制实战：清理输入文本中的制表符 `\t`。
// - 99% 的正常请求不含 `\t`：直接返回 Cow::Borrowed(raw)，零堆内存分配；
// - 仅 1% 的特殊请求含有 `\t`：按需分配新 String 替换字符，返回 Cow::Owned(replaced)。
fn sanitize_log_line<'a>(raw: &'a str) -> Cow<'a, str> {
    if raw.contains('\t') {
        // 触发写时复制：仅在确实需要修改时才在堆上分配 String
        Cow::Owned(raw.replace('\t', " "))
    } else {
        // 零拷贝快速路径：直接包装原切片引用返回
        Cow::Borrowed(raw)
    }
}

// 消费方函数：因为 Cow<'a, str> 实现了 Deref<Target = str>，
// 下游所有接收 &str 的函数都可以直接无感接收 &Cow<'_, str>
fn write_audit_entry(line: &str) {
    println!("  -> 写入审计日志: `{}`", line);
}
// ANCHOR_END: sanitize_cow

// ANCHOR: cow_to_mut
// 演示 Cow::to_mut() 的按需克隆能力：
// 如果当前是 Borrowed，首次调用 to_mut() 会自动在堆上克隆出一份 Owned 数据并返回 &mut；
// 如果当前已经是 Owned，则直接在原地修改，绝不二次分配。
fn append_warning_if_needed(text: &mut Cow<'_, str>, is_suspicious: bool) {
    if is_suspicious {
        text.to_mut().push_str(" [WARN: 异常请求]");
    }
}
// ANCHOR_END: cow_to_mut

fn main() {
    // 场景 1：干净输入（走零堆分配路径）
    let clean_input = "GET /api/orders 200 OK";
    let res1 = sanitize_log_line(clean_input);
    println!(
        "[干净输入] 是否借用原内存 (零分配): {}",
        matches!(res1, Cow::Borrowed(_))
    );
    write_audit_entry(&res1); // Deref Coercion 自动将 &Cow<str> 转为 &str

    // 场景 2：包含制表符的输入（按需触发一次堆分配）
    let dirty_input = "POST\t/api/login\t401";
    let res2 = sanitize_log_line(dirty_input);
    println!(
        "\n[需转义输入] 是否分配了新堆内存: {}",
        matches!(res2, Cow::Owned(_))
    );
    write_audit_entry(&res2);

    // 场景 3：使用 to_mut() 按需就地升级为拥有权并修改
    let mut msg: Cow<'_, str> = Cow::Borrowed("连接自 10.0.0.8");
    append_warning_if_needed(&mut msg, false); // 未修改，依然保持 Borrowed 零分配
    assert!(matches!(msg, Cow::Borrowed(_)));

    append_warning_if_needed(&mut msg, true); // 首次修改，to_mut() 自动克隆升级为 Owned
    assert!(matches!(msg, Cow::Owned(_)));
    println!("\n[to_mut 按需修改] 最终内容: `{}`", msg);
}
// ANCHOR_END: full
