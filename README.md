# Rust Workbook

Mastery challenges and project briefs aligned to the [Brown interactive Rust Book](https://rust-book.cs.brown.edu/).

Every task is written for Rust's type and ownership model. The workbook does not copy exercise text or solutions from another source.

## How to work

1. Create a Cargo project for a challenge when it needs more than a single function: `cargo new challenge_001`.
2. Before coding, write down the inputs, outputs, and ownership decisions.
3. Build a small working version, then add the listed edge cases.
4. Run `cargo fmt` and `cargo check` as part of your normal workflow. Add tests when the brief asks for them.
5. Keep solutions in your own project. These briefs intentionally do not include answers.

Each file in `chapters/` follows the number and scope of a concept-bearing book chapter. Each challenge includes an example plus numbered unit and integration test cases.

The workbook intentionally omits the book Introduction, Chapter 2 guessing-game walkthrough, and Chapter 21 web-server walkthrough. They are orientation or practice builds rather than new core material.

Chapter 12 remains because its project introduces practical CLI arguments, file I/O, environment variables, `stderr`, modular design, and test-driven development.

The [`projects/`](projects/README.md) track combines several topics in larger, practical builds. It contains 10 easy, 10 medium, and 10 hard project briefs.

Work through the projects by scope when you want integration practice rather than a topic-specific exercise.

## Chapters

- Book Chapter 1 — [Getting Started](chapters/01-getting-started.md)
- Book Chapter 3 — [Common Programming Concepts](chapters/03-common-programming-concepts.md)
- Book Chapter 4 — [Understanding Ownership](chapters/04-understanding-ownership.md)
- Book Chapter 5 — [Using Structs to Structure Related Data](chapters/05-using-structs.md)
- Book Chapter 6 — [Enums and Pattern Matching](chapters/06-enums-and-pattern-matching.md)
- Book Chapter 7 — [Packages, Crates, and Modules](chapters/07-packages-crates-and-modules.md)
- Book Chapter 8 — [Common Collections](chapters/08-common-collections.md)
- Book Chapter 9 — [Error Handling](chapters/09-error-handling.md)
- Book Chapter 10 — [Generic Types, Traits, and Lifetimes](chapters/10-generics-traits-and-lifetimes.md)
- Book Chapter 11 — [Writing Automated Tests](chapters/11-writing-automated-tests.md)
- Book Chapter 12 — [I/O Command-Line Project](chapters/12-io-command-line-project.md)
- Book Chapter 13 — [Iterators and Closures](chapters/13-iterators-and-closures.md)
- Book Chapter 14 — [More About Cargo](chapters/14-more-about-cargo.md)
- Book Chapter 15 — [Smart Pointers](chapters/15-smart-pointers.md)
- Book Chapter 16 — [Fearless Concurrency](chapters/16-fearless-concurrency.md)
- Book Chapter 17 — [Async, Await, Futures, and Streams](chapters/17-async-await-futures-and-streams.md)
- Book Chapter 18 — [Object-Oriented Features](chapters/18-object-oriented-features.md)
- Book Chapter 19 — [Patterns and Matching](chapters/19-patterns-and-matching.md)
- Book Chapter 20 — [Advanced Features](chapters/20-advanced-features.md)

## Test-case convention

Chapters before Chapter 11 use plain verification checklists because the learner has not studied Rust testing yet.

From Chapter 11 onward, and in every project brief, tests are separated into unit and integration cases. Codes such as `T01` identify normal behavior, boundaries, edge cases, corner cases, and failure paths.

## Projects

- [Easy projects](projects/easy/README.md)
- [Medium projects](projects/medium/README.md)
- [Hard projects](projects/hard/README.md)

## Real-world usage

The [real-world Rust challenges](real-world-usage/README.md) cover 10 production domains with company context, interfaces, operational constraints, test plans, deployment targets, and acceptance criteria.
