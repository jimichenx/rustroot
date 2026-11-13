# 课程代码实验

这里是各章节正文引用的实验代码，组织为一个统一的 Cargo workspace。

```
examples/
├── Cargo.toml                 workspace 根配置（edition = "2024"）
└── <part>/<chapter-group>/    按篇章与主题组织的 crate
    ├── Cargo.toml
    └── src/bin/*.rs           每个独立实验一个 bin
```

运行与校验：

```bash
cd site/examples
cargo build                                       # 校验全部实验源码可编译
cargo run -p ownership --bin moved-value          # 运行单个实验
```

约定：

1. **职责单一、与页面同源**：网站页面通过 `?raw` 导入源码并配合 `<Code>` 渲染，不在 MDX 里手抄同一份实验程序。
2. **分步切片（`ANCHOR`）**：当正文需要按推导步骤展示局部代码、最后核对完整可运行源码时，在 `.rs` 中使用成对的 `// ANCHOR: <name>` 与 `// ANCHOR_END: <name>` 标记区间，MDX 中写 `<Code code={src} lang="rust" anchor="<name>" />` 提取（构建插件会自动剥离标记行并对齐缩进，不在 MDX 里手写 `.split()`）。
3. **编译失败实验的处理**：故意触发编译错误的对照行默认注释掉，并在文件顶部注明预期错误码（如 `// Expect: E0382`），确保 `cargo build` 在全量校验与 CI 中始终通过。
4. **中英文站点共用**：源码中不写复述正文推导的大段中文教学注释；只保留文件头 `// Expect: EXXXX`、`unsafe` 块前必要的英文 `// SAFETY:` 说明及极简英文标识，原理解释一律写在页面正文中。
