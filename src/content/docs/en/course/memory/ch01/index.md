---
title: From automatic reclamation to ownership
description: Without syntax clutter, derive an automatic memory reclamation model from stack and heap mechanics—requiring neither a runtime garbage collector nor manual free calls.
id: ownership
---

This chapter addresses an apparent contradiction: **if programmers never write `free` by hand, and no garbage collector (GC) scans memory or counts references at runtime, how can a program reclaim memory automatically while remaining memory-safe?**

To keep syntax out of the way, the six sections ahead form one continuous derivation from first principles. For the first five sections, we write no real Rust syntax and hold back top-level rules such as "ownership" and "borrowing." Instead, we take the perspective of a language designer—starting from stack and heap mechanics, letting each safety flaw force out the next rule, and naming specific mechanisms only after they emerge. The closing section maps the completed model onto Rust's standard terminology.

The derivation advances in three stages:

- **Framing the problem and the memory layout.** Starting from the hard triangle of language design, we lock in the target—automatic reclamation without a runtime collector—and inspect how stack frames and heap blocks differ in lifetime to find a static cue for heap cleanup.
- **From invalidating the old variable to move semantics.** We start with the simplest rule—free the heap block when its stack variable leaves scope—watch a single assignment expose a double-free conflict, and see how invalidating the source variable becomes move semantics while hardware still copies stack bytes bitwise.
- **From restricted pointers to compile-time `free` insertion.** To avoid moving arguments on read-only calls, we pass a restricted address (a reference), compare valid spans (lifetimes) at compile time to rule out dangling references, and finally align the whole chain with ownership and borrowing: letting the compiler insert `free` at the exact right spot.

When you finish this chapter—without memorizing a single syntax rule—you should be able to reason through three questions from scratch:

1. Why do local stack variables disappear automatically on return, whereas heap allocations must be explicitly returned to the allocator?
2. Why does `=` copy an integer yet move a heap-backed string—and why does a move require no physical movement of bytes in memory?
3. Why can read-only references be copied freely, yet still require the compiler to restrict where they remain valid?
