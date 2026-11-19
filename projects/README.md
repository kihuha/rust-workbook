# Rust projects

These projects combine several Rust Book chapters in one practical build. Every project names the exact book sections to study before starting.

## Difficulty levels

| Level | Project scope | Brown Rust Book prerequisite |
| --- | --- | --- |
| [Easy](easy/README.md) | One package, a small domain model, local input/output, and focused tests | Chapter 11 plus the exact Chapters 3–13 sections listed for the chosen project |
| [Medium](medium/README.md) | Multiple modules plus persistence, threads, async I/O, or richer boundaries | Chapters 3–17; complete two easy projects and study each project’s linked sections |
| [Hard](hard/README.md) | Protocols, recovery, schedulers, storage, bounded concurrency, or advanced APIs | Relevant chapters through 20; complete two medium projects and review the linked advanced sections |

## Recommended learning path

| If this is difficult | Learn or practise next |
| --- | --- |
| The compiler reports frequent move or borrow errors | Revisit [Chapter 4](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html). |
| The data model contains many booleans and invalid combinations | Revisit [Chapters 5–6](https://rust-book.cs.brown.edu/ch05-00-structs.html). |
| Parsing and validation are mixed with `println!` calls | Revisit [Chapters 7, 9, and 12](https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html). |
| Reports require repeated manual loops | Revisit [Chapter 13](https://rust-book.cs.brown.edu/ch13-00-functional-features.html). |
| Shared or concurrent state is unclear | Revisit [Chapters 15–17](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html). |
| Changes regularly break working behavior | Add tests using [Chapter 11](https://rust-book.cs.brown.edu/ch11-00-testing.html). |

## Workbook prerequisite map

| Skill area | Study material |
| --- | --- |
| Ownership, borrowing, references, and slices | [Book Chapter 4](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html) |
| Structs, enums, and domain modeling | [Book Chapters 5–6](https://rust-book.cs.brown.edu/ch05-00-structs.html) |
| Packages, crates, modules, and public APIs | [Book Chapter 7](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) |
| Vectors, strings, and hash maps | [Book Chapter 8](https://rust-book.cs.brown.edu/ch08-00-common-collections.html) |
| Recoverable and unrecoverable errors | [Book Chapter 9](https://rust-book.cs.brown.edu/ch09-00-error-handling.html) |
| Generics, traits, and lifetimes | [Book Chapter 10](https://rust-book.cs.brown.edu/ch10-00-generics.html) |
| Unit and integration testing | [Book Chapter 11](https://rust-book.cs.brown.edu/ch11-00-testing.html) |
| CLI, files, environment, and `stderr` | [Book Chapter 12](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html) |
| Closures and iterator pipelines | [Book Chapter 13](https://rust-book.cs.brown.edu/ch13-00-functional-features.html) |
| Workspaces and package quality | [Book Chapter 14](https://rust-book.cs.brown.edu/ch14-00-more-about-cargo.html) |
| Shared ownership and interior mutability | [Book Chapter 15](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html) |
| Threads, channels, shared state, and async | [Book Chapters 16–17](https://rust-book.cs.brown.edu/ch16-00-concurrency.html) |
| Trait objects and design tradeoffs | [Book Chapter 18](https://rust-book.cs.brown.edu/ch18-00-oop.html) |
| Advanced patterns, unsafe code, traits, types, and macros | [Book Chapters 19–20](https://rust-book.cs.brown.edu/ch19-00-patterns.html) |

## How to use the briefs

1. Create a separate Cargo package for each project.
2. Build the smallest end-to-end flow first.
3. Keep domain logic separate from terminal, file, network, or database I/O.
4. Add tests for the examples and failure cases in the brief.
5. Finish the required behavior before attempting stretch goals.

## Test-case convention

Each project includes numbered unit and integration cases. Implement the normal path first, then boundaries, edge cases, corner cases, and failures. Preserve the `T01…` codes in test names or comments for traceability.

Output wording may differ from the examples unless a format is explicitly required. The data and behavior should still match.
