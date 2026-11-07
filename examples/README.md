# 课程代码实验

这里是课节引用的实验代码，是一个 Cargo workspace。课节不必套「预测 → 尝试 → 解释」。

```
examples/
├── Cargo.toml                 workspace
└── <part>/<lesson-slug>/      每课一个 crate，与网站 URL /<part>/<lesson-slug>/ 对应
    ├── Cargo.toml
    └── src/bin/*.rs           每个实验一个 bin，文件名说明验证的概念
```

运行：

```bash
cd examples
cargo build                                  # 全部
cargo run -p hard-triangle --bin moved-value # 单个实验
```

约定：

- 每个实验只验证一个概念，代码不超过 10~15 行。
- 关注编译器说了什么，而不是程序输出了什么。
- 故意编译失败的例子默认注释掉并在文件头说明预期错误码，读者手动取消注释运行；这样 `cargo build` 在 CI 里始终能通过。
- 网站页面从这里引用源码（`?raw` import + Starlight `<Code>`），不要在 MDX 里手抄一份。
