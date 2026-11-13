---
title: 工程与排障
description: 查阅 Rust 工具链全景地图（rustup / rustc / cargo / Edition）与高频编译错误码的诊断判据。
sidebar:
  order: 4
---

无论是刚配置好 Rust 环境被一串命令行工具弄晕，还是在日常编码与审查 AI 代码时面对大段编译器报错需要定位约束，都可以直接查阅本章。

这里收录了 `rustup` / `rustc` / `cargo` 的工具链角色全景与十类高频编译错误码的排障判据（关于多包 Workspace 与 Features 条件编译机制，详见主线[多包工作空间与条件编译](/architecture/cargo-and-features/)）。各节不必按顺序阅读。
