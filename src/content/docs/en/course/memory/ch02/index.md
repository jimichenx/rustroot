---
title: Variables and assignment
description: Put the ownership and borrowing model from Chapter 1 to work in real Rust code by tracing cleanup duty across declarations, reassignment, and shadowing.
id: variables
---

In most programming languages, variable declarations and assignments barely warrant a dedicated chapter: you declare a variable, put a value in it with `=`, and read or replace it whenever you like. When learning Rust, however, many people write their very first lines of `let` and `=` and immediately run into compiler errors about moved values, `Copy` versus `Move`, or not being allowed to call a method without `mut`.

Chapter 1 showed why: in garbage-collected languages, `=` merely copies a value or an object reference while the runtime handles reclamation behind the scenes; in Rust, every `let` and `=` hands off or triggers the **cleanup responsibility** we derived in Chapter 1 at compile time.

This chapter leaves pseudocode behind. We open the compiler and trace our memory model through the most fundamental Rust syntax—`let` and `=`—in three steps:

- **Step 1: Initial binding (`let x = ...`)**—creating a new variable to take on an initial value. Starting from our first runnable program, we verify why the same `=` splits into three paths—Copy, Move, and borrowing—across integers, `String`, and references (`&T`).
- **Step 2: Reassignment (`x = ...`)**—updating an existing variable. We unpack the two permissions unlocked by `let mut` and follow the compiler's state tracking to answer where the old value goes when a new value takes its place.
- **Step 3: Shadowing (writing `let x = ...` again)**—creating a new variable with the same name. We contrast reassignment with shadowing and verify through an experiment whether a shadowed value is dropped on the spot.

By the end of this chapter, when looking at a dozen lines of Rust code—without memorizing compiler error codes—you should be able to answer three questions line by line:

1. **Which action is this**: is this line creating a new variable (initial `let`), updating an existing variable (reassignment), or shadowing an old variable with a new `let` of the same name?
2. **How is the right-hand side handed off**: is the value on the right copied, moved, or borrowed by address?
3. **When is the old value freed**: which variables are still valid right now, which variable owns each heap buffer, and is that buffer freed right at the assignment line or at the end of the scope (`}`)?
