---
title: Memory and ownership
description: Derive ownership and borrowing from how stack and heap memory are reclaimed, then verify cleanup responsibility in Rust declarations, assignments, and shadowing.
---

Whether you are coming from another language or have already written some Rust, terms like ownership, move, reference, and lifetime often make sense on their own yet feel like an arbitrary list of rules when combined. Why does assigning a `String` invalidate the original variable? Why can a caller no longer use a value after passing it to a function? Why must a reference never outlive its owner?

Rather than asking you to memorize rules, this part starts from Rust's core constraint: guaranteeing memory safety without a garbage collector (GC) and without manual `free` calls. Every rule above is forced out of physical memory by that single goal. We begin by rebuilding the underlying memory picture—what a function call leaves on the stack, where dynamically allocated heap data sits after a call returns, and how stack frames, heap blocks, and addresses connect.

If your background is Java, Python, or Go, you rarely have to think about that layer—not because the stack and heap are absent, but because a garbage-collected runtime handles reclamation behind the scenes. In Rust, no background collector steps in; understanding who is responsible for freeing memory is part of writing the code.

This part leaves the broader type system, mutable references, concurrency, and `unsafe` for later. How types encode constraints, how read and write access are separated, and how multiple owners share data build on the foundation established here.

Two chapters make up this part. [From automatic reclamation to ownership](/memory/ownership/) sets syntax aside and derives an automatic cleanup model directly from stack and heap mechanics; [Variables and assignment](/memory/variables/) brings that model into real Rust code to trace cleanup responsibility across declarations, reassignment, and shadowing.

By the end of this part, an ownership, argument-passing, or borrowing error in single-threaded code should no longer be something you patch by trial and error—you will be able to trace why the compiler rejected it from memory layout and cleanup responsibility.
