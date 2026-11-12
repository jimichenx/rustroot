---
title: 生产级工程基建：错误处理与日志追踪
description: 打通标准库 Error 契约与工业界事实标准库，掌握库层 thiserror 与应用层 anyhow 的分层错误架构，以及从传统日志门面演进到异步结构化追踪 tracing 的可观测性体系。
id: error-and-observability
---

走完模块封装、类型驱动设计与编译期宏之后，如果此刻让你从零搭建一个生产级 Rust 后端服务或命令行工具，你立刻会遇到两件每天都要打交道的基础设施选型：

1. **几十种异构错误怎么分层往上抛？** 当一个顶层接口同时调用了数据库查询、HTTP 客户端与 JSON 解析，难道要为每一层业务函数都手写一大堆 `enum`、`Display` 和 `From` 转换样板吗？社区常说的 **`thiserror`** 和 **`anyhow`** 到底是怎么分工的？
2. **线上日志与链路上下文怎么记？** 在 Java 中有 `SLF4J + Log4j2 / Logback` 以及存放请求 ID 的 `MDC（ThreadLocal）`；到了没有内置日志器的 Rust 标准库、尤其是进入第 14 章的多线程异步 `Future` 运行时之后，为什么传统的平面日志与线程局部变量不再够用，全生态为什么转向了 **`tracing`**？

本章合流前十六章建立的枚举（第 6 章）、转换契约与极小内核（第 7 章）、闭合枚举 vs 开放动态分发（第 8 章）、异步状态机（第 14 章）与过程宏（第 16 章），分两节把这两大生产支柱彻底立稳：

1. **工业级错误处理架构（`Error` 契约、`thiserror` 与 `anyhow`）**：先看清标准库 `std::error::Error` 契约的因果链底座与 `Box<dyn Error>` 的局限，再从「闭合集合 vs 开放类型擦除」的第一性原理出发，划清库与领域内层用 `thiserror`、应用与接口顶层用 `anyhow` 的分层边界。
2. **从 `Log4j` 到 `tracing`（门面解耦、结构化键值与异步 `Span` 追踪）**：对照 Java 日志门面理解 Rust 库与收集器（`Subscriber`）的解耦机制，推演为什么 `ThreadLocal` 上下文在 `async / await` 跨线程轮询下会串台失效，掌握 `tracing` 的结构化键值与 `Span` 区间追踪，并收束错误传播与日志记录不重复刷屏的协同铁律。

## 读完本章，检验自己的三个问题

- 标准库的 `std::error::Error` Trait 要求实现哪两个前置契约，它的 `.source()` 方法解决什么问题？
- 在同一个工程里，什么时候应该用 `thiserror` 定义强类型枚举，什么时候应该用 `anyhow::Result<T>` 擦除具体错误类型并挂载 `.with_context(...)`？
- 为什么在多线程 `async / await` 运行时中不能像传统同步模型那样用 `ThreadLocal` 传递 `request_id`？遇到底层报错时，为什么内层函数不应该一边调 `error!` 打日志一边把 `Err` 向上返回？
