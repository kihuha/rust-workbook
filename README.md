# Rust Workbook

Mini briefs for practicing Rust through small, runnable projects. The tasks follow the learning order in the personal Rust plan and cover a broad mix of practical and puzzle-style exercises.

Every task is written for Rust's type and ownership model. The workbook does not copy exercise text or solutions from another source.

## How to work

1. Create a Cargo project for a challenge when it needs more than a single function: `cargo new challenge_001`.
2. Before coding, write down the inputs, outputs, and ownership decisions.
3. Build a small working version, then add the listed edge cases.
4. Run `cargo fmt` and `cargo check` as part of your normal workflow. Add tests when the brief asks for them.
5. Keep solutions in your own project. These briefs intentionally do not include answers.

Each file in `chapters/` contains three numbered challenges. The sequence number is global, so it is easy to track progress across chapters. Challenge sizes are intentionally modest; most are designed for one focused sitting.

The [`projects/`](projects/README.md) track combines several topics in larger, practical builds. It contains 10 easy, 10 medium, and 10 hard project briefs.

Work through the projects by scope when you want integration practice rather than a topic-specific exercise.

## Chapters

1. Ownership, moves, and cloning
2. Immutable and mutable borrowing
3. Reference validity, scopes, and non-lexical lifetimes
4. Lifetime relationships
5. Structs, enums, and pattern matching
6. Collections, strings, and slices
7. Generics and traits
8. Error handling
9. Modules and Cargo
10. Closures and iterators
11. Smart pointers and interior mutability
12. Concurrency and async/await
13. Testing and idiomatic Rust

The lifetime chapter begins with validity reasoning. Move to explicit lifetime syntax only after the earlier borrowing work feels solid.

## Projects

- [Easy projects](projects/easy/README.md)
- [Medium projects](projects/medium/README.md)
- [Hard projects](projects/hard/README.md)
