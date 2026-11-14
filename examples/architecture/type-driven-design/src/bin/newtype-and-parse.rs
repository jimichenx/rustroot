// Expect: E0308 (when uncommenting mixed UserId and OrderId arguments)
// ANCHOR: full
use std::fmt;

// ANCHOR: id_newtypes
// 1. Newtype via single-field tuple struct:
// distinct types at compile time, same 8-byte layout as u64 at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderId(pub u64);

fn bind_order_to_user(user: UserId, order: OrderId) {
    println!("[参数防呆] 成功将订单 {:?} 绑定至用户 {:?}", order, user);
}
// ANCHOR_END: id_newtypes

// ANCHOR: parse_dont_validate
// 2. 结合 15.1 的 mod 私有字段边界，落地「解析而非校验（Parse, don't validate）」
pub mod account {
    // 结构体公开，但内部的 String 字段保持私有（未标 pub）
    // 模块外的任何代码都无法用 Email("bad_input".into()) 直接拼装实例
    #[derive(Debug, Clone)]
    pub struct Email(String);

    impl Email {
        // 唯一受控入口：将原始未校验的 String「解析」提升为受信任的 Email 类型
        pub fn parse(raw: String) -> Result<Self, &'static str> {
            let trimmed = raw.trim();
            if trimmed.is_empty() || !trimmed.contains('@') {
                return Err("邮箱格式非法：必须非空且包含 `@` 符号");
            }
            Ok(Self(trimmed.to_string()))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
}

// 下游核心业务函数：参数直接索要 &account::Email。
// 函数体内部无需再写任何防御性校验，因为非法的字符串在编译期根本造不出 Email 实例
fn send_welcome_email(addr: &account::Email) {
    println!("[解析而非校验] 无需重复检查格式，直接向合法地址 `{}` 发送欢迎邮件", addr.as_str());
}
// ANCHOR_END: parse_dont_validate

// ANCHOR: orphan_rule_wrapper
// 3. 使用本地 Newtype 包装外部容器 Vec<String>，合法跨越孤儿规则实现外部 Display Trait
pub struct PathList(pub Vec<String>);

impl fmt::Display for PathList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(" -> "))
    }
}
// ANCHOR_END: orphan_rule_wrapper

fn main() {
    let uid = UserId(1001);
    let oid = OrderId(9527);
    bind_order_to_user(uid, oid);

    // 验证零运行时内存开销：Newtype 与底层 u64 的内存大小完全一致（同为 8 字节）
    assert_eq!(std::mem::size_of::<UserId>(), std::mem::size_of::<u64>());

    /*
    // 如果手滑把订单 ID 和用户 ID 的传参顺序写反了：
    // 编译错误 E0308：mismatched types (expected `UserId`, found `OrderId`)
    bind_order_to_user(oid, uid);
    */

    // 在系统入口处一次性完成解析，后续业务全程传递强类型 Email
    let valid_email = account::Email::parse(String::from("  alice@example.com ")).unwrap();
    send_welcome_email(&valid_email);

    let bad_attempt = account::Email::parse(String::from("invalid-email"));
    println!("[边界拦截] 非法输入被唯一构造入口拦下: {:?}", bad_attempt);

    // 验证跨越孤儿规则的 Display 格式化输出
    let route = PathList(vec![
        String::from("/home"),
        String::from("/course"),
        String::from("/architecture"),
    ]);
    println!("[跨越孤儿规则] 路由路径: {}", route);
}
// ANCHOR_END: full
