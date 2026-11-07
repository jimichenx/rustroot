---
title: Memory and ownership
description: Derive ownership and borrowing from memory facts, then test the model with Rust declarations, assignments, and shadowing.
---

You may already write another language, or you may have spent some time with Rust. Ownership, moves, references, and lifetimes can make sense individually yet feel like a list of prohibitions together: why does assigning a `String` invalidate the original variable, why can the caller no longer use a value after passing it to a function, and why must a reference not outlive its owner?

This part starts with the memory facts behind those questions. What does a function call place on the stack? What happens to heap data after the call returns? How do variables, pointers, and bytes relate? We derive the rules from these facts rather than asking you to memorize them.

If you come from Java or Python, you may not have needed this view of memory. Their runtimes handle details you rarely have to consider directly. Rust asks you to understand those details when deciding how values can be used.

This part does not explore the type system, mutable references, concurrency, or `unsafe`. How types express these rules, how reading and writing coexist, and how multiple owners share data are questions for later parts.

There are two chapters. [From automatic reclamation to ownership](/memory/ownership/) derives the model through six sections, each addressing a problem left by the previous one. [Variables and assignment](/memory/variables/) applies it to code, distinguishing declarations, initialization, reassignment, and shadowing.

By the end, you should be able to explain the reasoning and write a short program of your own: determine which variables remain usable, who frees a string's memory, and whether a new declaration with the same identifier changes responsibility for the old value.
