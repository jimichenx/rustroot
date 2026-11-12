// ANCHOR: full

// ANCHOR: business_logic
/// 计算扣除手续费后的实际到账金额（单位：分）。
///
/// # 文档即测试（Doc Test）示例
/// 下面的代码块会被 `cargo test` 自动提取当作外部调用代码编译并运行：
/// ```
/// let net = settle_amount(1000, 30).unwrap();
/// assert_eq!(net, 970);
/// ```
pub fn settle_amount(gross_cents: u64, fee_bps: u64) -> Result<u64, &'static str> {
    if gross_cents == 0 {
        return Err("结算金额不能为 0");
    }
    let fee = compute_internal_fee(gross_cents, fee_bps);
    Ok(gross_cents - fee)
}

// 私有辅助函数（未标 pub）：外部模块不可见，但底部的子模块 `mod tests` 可直接访问
fn compute_internal_fee(gross_cents: u64, fee_bps: u64) -> u64 {
    assert!(fee_bps <= 10_000, "费率基点不能超过 10000 (100%)");
    (gross_cents * fee_bps) / 10_000
}
// ANCHOR_END: business_logic

// ANCHOR: unit_tests
// 仅在执行 `cargo test` 时编译该子模块；常规 `cargo build` 时整块剥离，零二进制残留
#[cfg(test)]
mod tests {
    // 将父模块（当前文件）中的所有公开与私有条目引入子模块作用域
    use super::*;

    // 1. 直接测试父模块的私有函数（顺应 15.1「子模块看父模块全透明」的规则）
    #[test]
    fn test_private_fee_calculation() {
        assert_eq!(compute_internal_fee(10_000, 25), 25);
        assert_eq!(compute_internal_fee(500, 0), 0);
    }

    // 2. 返回 Result<(), E> 的测试函数：允许在测试体内部直接使用 `?` 传播错误
    #[test]
    fn test_settle_amount_ok() -> Result<(), &'static str> {
        let net = settle_amount(10_000, 25)?;
        assert_eq!(net, 9_975);
        Ok(())
    }

    // 3. 验证非法输入是否按预期触发 panic!
    #[test]
    #[should_panic(expected = "费率基点不能超过 10000")]
    fn test_excessive_fee_panics() {
        compute_internal_fee(1_000, 15_000);
    }
}
// ANCHOR_END: unit_tests

fn main() {
    let net = settle_amount(10_000, 25).unwrap();
    println!("结算成功：原始金额 10000 分，扣除费率后实收 {} 分", net);
    println!("请运行 `cargo test --bin testing-system` 验证内置的单元测试模块。");
}
// ANCHOR_END: full
