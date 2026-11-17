---
title: Variables and assignment
description: Bring the ownership and borrowing model into real Rust code, tracing the compile-time state table and cleanup responsibility across variable declarations, reassignment, and shadowing.
id: variables
---

In most mainstream languages, declaring and assigning variables barely calls for discussion: you define a variable, store a value with `=`, and read or replace it whenever you want. Early in Rust, however, the very first lines of code trigger a stream of compiler rejections—why an original variable becomes unusable right after assignment, or why a newly declared variable forbids mutating its contents by default.

[From automatic reclamation to ownership](/memory/ownership/) established the underlying reason: in garbage-collected (GC) languages, `=` copies a value or an object reference while the runtime handles reclamation behind the scenes; in Rust, every `let` and `=` transfers or triggers **cleanup responsibility** at compile time.

This chapter opens the compiler. Starting from your first runnable Rust program, we follow the three variable operations that appear throughout everyday code: defining a new variable with `let x = ...` across copy, move, and borrowing; reassigning an existing variable with `x = ...` to inspect mutability permissions and where the old value goes; and repeating `let x = ...` to declare a new variable of the same name while tracing scope and drop timing.

When you finish this chapter—without memorizing compiler error codes—you should be able to read real Rust code line by line and answer three questions:

1. **Which action is this**: does this line define a new variable (initial `let`), update the same existing variable (reassignment without `let`), or declare another new variable with the same name (a second `let`)?
2. **How is the right-hand side handed off**: is the value on the right copied, moved, or borrowed by address?
3. **When is the old value dropped**: which variables remain valid right now, which variable owns each heap allocation, and is that memory dropped at the assignment line or at scope exit (`}`)?
