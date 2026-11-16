---
title: From automatic reclamation to ownership
description: Without syntax clutter, derive an automatic memory reclamation model from stack and heap mechanics—requiring neither a runtime garbage collector nor manual free calls.
id: ownership
---

This chapter addresses an apparent contradiction: **if programmers never write `free` by hand, and no garbage collector (GC) scans memory or counts references at runtime, how can a program reclaim memory automatically while remaining memory-safe?**

To keep syntax out of the way, the six sections ahead form one continuous derivation from first principles. For the first five sections, we write no real Rust syntax and introduce no Rust-specific jargon such as "ownership," "borrowing," or "lifetimes." Instead, we take the perspective of a language designer—starting from stack and heap mechanics and letting each safety flaw force out the next rule. The closing section maps the completed model directly onto Rust's standard terminology.

The derivation advances in three stages:

- **Framing the problem and the memory layout.** Starting from the hard triangle of language design, we lock in the target—automatic reclamation without a runtime collector—and inspect the stack and the heap. Stack frames are reclaimed automatically when a function returns, leaving only dynamically allocated heap blocks in need of cleanup. Crucially, the stack variable holding a heap pointer has a deterministic scope exit (`}`).
- **From invalidating the old variable to move semantics.** We start with the simplest rule—free the heap block when its stack variable leaves scope—and watch a single assignment `t = s` trigger a double free. Fixing that flaw first requires invalidating the source variable, which we then reshape into an intuitive model of copy versus move semantics before looking under the hood: hardware still copies stack bytes bitwise, while invalidation and cleanup transfer happen entirely in the compiler's state table.
- **From restricted pointers to compile-time `free` insertion.** Because moving values on every function call makes read-only parameters impractical, we pass an eight-byte stack address instead, strip away the unchecked freedoms of C raw pointers to form references, and compare valid spans at compile time to rule out dangling references. Finally, we align the whole chain with ownership and borrowing to reveal their single shared job: letting the compiler insert `free` at the exact right spot.

When you finish this chapter—without memorizing a single syntax rule—you should be able to reason through three questions from scratch:

1. Why do local stack variables disappear automatically on return, whereas heap allocations must be explicitly returned to the allocator?
2. Why does `=` copy an integer yet move a heap-backed string—and why does a move require no physical movement of bytes in memory?
3. Why can read-only references be copied freely, yet still require the compiler to restrict where they remain valid?
