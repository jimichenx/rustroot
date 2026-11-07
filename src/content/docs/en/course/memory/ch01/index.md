---
title: From automatic reclamation to ownership
description: Design a model for automatic memory reclamation without a runtime garbage collector or manual free calls.
id: ownership
---

This chapter tackles an apparent paradox: **if the programmer never writes `free` by hand, and no garbage collector (GC) scans or counts references at runtime, how can a program reclaim memory automatically while staying completely safe?**

To keep syntax noise out of the way, the six sections in this chapter form a continuous **first-principles derivation**. Across the first five sections, we set aside real Rust syntax and terminology like "ownership" or "borrowing," building the reclamation model step by step from physical memory constraints; the final section maps the resulting mechanisms onto Rust's native concepts and takes stock of the remaining gaps. Starting in Chapter 2, we open the compiler and test this model in real Rust code.

The six sections unfold in three stages:

- **Stage 1: Frame the problem and the mental map (Sections 1–2).** The hard triangle locates the breakthrough direction—automatic reclamation without GC. The stack and the heap then establish the physical layout: stack frames vanish automatically when a function returns, so only heap resources need reclamation—and the stack variable controlling a heap resource happens to have a deterministic scope exit (`}`).
- **Stage 2: Build the minimal model and derive Move (Section 3).** Starting from the rule "free the heap when the stack variable leaves scope," a single assignment `t = s` exposes a double-free bug, forcing out the requirement that only one variable can be responsible for freeing a heap block at a time—and splitting `=` into Copy and Move semantics.
- **Stage 3: Add references and dangling-reference checks, then unify with Rust (Sections 4–6).** A Move-only model invalidates arguments even on read-only function calls, prompting us to design a reference that holds an address without taking responsibility for cleanup. To prevent dangling references when the original variable exits scope first, the compiler checks lifetime constraints. The final section matches the model to Rust's ownership, borrowing, and lifetime concepts.

By the end of this chapter—without memorizing a single syntax rule—you should be able to explain three things in your own words:
1. Why does the stack clean itself up while heap resources require explicit reclamation?
2. Why does the same assignment `=` leave both integer variables usable while invalidating the old variable for a heap-backed string?
3. Why can read-only references be copied and shared, yet require compile-time lifetime constraints?
