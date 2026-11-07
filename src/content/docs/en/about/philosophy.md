---
title: Philosophy
description: An open guide that starts from runtime constraints and re-derives Rust's language mechanisms.
sidebar:
  order: 1
---

## Re-deriving Rust from runtime constraints

Rust Root is not a Rust syntax manual or an API reference. It is an open guide that starts from the constraints a program faces at runtime and re-derives Rust's language mechanisms.

The official Book teaches you how to use Rust. Rust Root explains why Rust is designed this way. This guide starts with the problems behind ownership, borrowing, and lifetimes, then builds a model you can carry into unfamiliar code.

Rust Root and the Book form a **two-track companion**:

| | This guide (thinking track) | The Book (practice track) |
|---|---|---|
| Question answered | "Why?" | "How?" |
| Deliverable | Mental models, design intuition, paradigms | Syntax, coding skill, practice |
| How you learn | Read, think, derive, run experiments | Read, type, build |

After a section, you may not be ready to write a complete program in that area. When you return to the Book, you will know what the syntax protects and what question to ask when the compiler rejects your code.

## How the guide proceeds

Each main thread follows four steps:

1. Pose a problem that a program cannot avoid;
2. Try a solution that appears to work;
3. Let that solution fail at a concrete boundary;
4. Derive Rust's mechanism from the failure, including the cost it introduces.

For example, we do not begin by defining ownership. We first ask: without a garbage collector, who frees heap memory? When sharing breaks the first release model, ownership and moves emerge as part of the solution.

This guide presents an organized line of reasoning, not a reconstruction of Rust's actual historical development. Each section also marks the boundary of its current model, so you can see why another mechanism is needed later.

## Code

A section may include a short program when a compiler decision or a runtime fact is easier to verify in code. Not every section needs one, and there is no fixed sequence of steps. Code experiments live under `examples/` in the repository; some include commented-out failing lines so you can uncomment them and inspect the compiler diagnostics directly. The Book remains the guide to writing Rust.

## Who it's for

This guide is for people who already know at least one programming language:

- **Dynamic languages (Python / JavaScript / Ruby)** — you may need to build a stack-and-heap model from scratch. The main path starts from this experience.
- **Managed languages (Java / Go / C#)** — you may need to rethink memory management outside garbage collection and revisit familiar abstraction boundaries.
- **Systems languages (C / C++)** — the main hazard is *false familiarity*: ownership is not RAII, traits are not virtual tables, and lifetimes are not pointer validity.

## What this guide does not cover

Full Rust syntax, application development, async/await, macros, `unsafe`, and domain-specific frameworks are outside the main path. The Book remains the place for syntax and practice; this guide builds the model that lets you see where those constraints come from.
