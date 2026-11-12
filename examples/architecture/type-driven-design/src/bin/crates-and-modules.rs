// Expect: E0451 (when uncommenting direct struct literal construction with private fields outside the module)
// ANCHOR: full

// ANCHOR: mod_tree_and_boundary
// 当前源文件自身就是根模块（crate 根）。
// 我们在内部定义一个名为 billing 的子模块，它既是一个命名空间，也是封装私有字段的容器边界。
mod billing {
    // 结构体标了 pub，外部模块可以看见 PaymentOrder 这个类型名字
    pub struct PaymentOrder {
        pub order_id: u64, // 公开字段：模块内外均可直接读写
        amount_cents: u64, // 私有字段（未标 pub）：仅限 billing 模块及子模块内部访问
    }

    impl PaymentOrder {
        // 公开关联函数：模块外的代码只能通过这个受控入口创建实例（通过 `::` 路径调用）
        pub fn try_new(order_id: u64, amount_cents: u64) -> Result<Self, &'static str> {
            if amount_cents == 0 {
                return Err("支付金额必须大于 0");
            }
            Ok(Self {
                order_id,
                amount_cents,
            })
        }

        // 公开方法：模块外通过 `.` 方法调用符号安全读取金额
        pub fn amount_cents(&self) -> u64 {
            self.amount_cents
        }
    }

    // 同一模块（billing）内的兄弟函数：与 PaymentOrder 是一家人，可直接读取其私有字段
    pub(crate) fn format_internal_receipt(order: &PaymentOrder) -> String {
        format!("内部凭据 [订单 #{}]: {} 分", order.order_id, order.amount_cents)
    }

    // 子模块 audit：子模块对父模块全透明，天然有权直接访问父模块的一切私有项与私有字段
    pub mod audit {
        use super::PaymentOrder;

        pub fn is_high_value(order: &PaymentOrder) -> bool {
            // 直接读取父模块 PaymentOrder 的私有字段 amount_cents
            order.amount_cents >= 100_000
        }
    }
}
// ANCHOR_END: mod_tree_and_boundary

// ANCHOR: pub_use_facade
// 使用 pub use 将内部子模块的核心类型与函数重导出（Re-export）到顶层门面
pub use billing::PaymentOrder;
pub use billing::audit::is_high_value;
// ANCHOR_END: pub_use_facade

fn main() {
    // 1. 清晰区分 `::`（编译期路径寻址）与 `.`（运行时实例成员/方法访问）：
    //    `PaymentOrder::try_new` 用 `::` 在类型命名空间下查找关联函数；
    //    `.unwrap()` 用 `.` 在返回的 Result 值实例上调用方法。
    let order = PaymentOrder::try_new(1001, 250_000).unwrap();

    println!(
        "订单 #{} 创建成功，金额 = {} 分，是否大额订单: {}",
        order.order_id,        // `.` 访问公开字段
        order.amount_cents(),  // `.` 调用实例方法
        is_high_value(&order), // 直接调用顶层 pub use 重导出的函数
    );

    // 调用 pub(crate) 限制在当前 crate 内部可见的辅助函数
    println!("{}", billing::format_internal_receipt(&order));

    /*
    // 如果在 billing 模块外部尝试绕过 try_new 校验、直接用字面量拼装结构体：
    // 编译错误 E0451：field `amount_cents` of struct `billing::PaymentOrder` is private
    let illegal_order = PaymentOrder {
        order_id: 1002,
        amount_cents: 0,
    };
    */
}
// ANCHOR_END: full
