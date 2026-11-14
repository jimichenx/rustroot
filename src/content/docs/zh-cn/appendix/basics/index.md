---
title: 基础语法
description: 主线示例里的基础写法集中在这里查：格式化打印、标量与元组、函数与表达式、struct 与 enum、模块与 Prelude。
draft: true
sidebar:
  order: 1
  badge:
    text: 草稿
    variant: caution
---

主线按问题往下推，不按语法表铺开。示例里会用到一些正文还没推导的写法。如果你卡在字面上——`println!` 怎么打印变量、`usize` 是什么、函数最后一行为什么不能加分号、`struct` 和 `enum` 怎么写、`String` 为什么不用 `use`——先在这里查清写法和最常见的编译报错，再回到原来的那一节。

这些内容不参加所有权、借用和类型的推演，也不必按顺序读完。每节只解决写法；谁释放、引用能不能修改、类型约束什么，仍以主线为准。
