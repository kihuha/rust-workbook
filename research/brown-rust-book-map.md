# Brown interactive Rust Book curriculum map

Research date: 2026-11-19

Primary source: [The Rust Programming Language, Brown interactive edition](https://rust-book.cs.brown.edu/), maintained by Brown University's Cognitive Engineering Lab.

## Scope and interpretation

The source contains an Introduction followed by numbered Chapters 1–21. Treating the Introduction as the first content unit gives the 22-unit sequence requested for this workbook.

The Brown edition describes itself as an experimental fork of the official Rust book. It keeps broadly similar content while adding interactive quizzes and substantially revised ownership explanations and visualizations.

The canonical sequence and section titles come from the repository's [official `SUMMARY.md`](https://github.com/cognitive-engineering-lab/rust-book/blob/main/src/SUMMARY.md). The live chapter pages linked below are the learning references.

## 22-unit curriculum sequence

### Unit 00 — Introduction

- **Source:** [Introduction](https://rust-book.cs.brown.edu/ch00-00-introduction.html)
- **Major sections:** Who Rust Is For; Who This Book Is For; How to Use This Book; Source Code.
- **Learning focus:** Understand Rust's goals, intended audiences, assumed background, and how the book combines explanation with hands-on code.
- **Mastery evidence for the workbook:** Explain why Rust emphasizes both performance and reliability; identify the local tools and study workflow needed for later chapters.
- **Built-in assessment:** No embedded quiz in the current source.

### Unit 01 — Getting Started

- **Source:** [Getting Started](https://rust-book.cs.brown.edu/ch01-00-getting-started.html)
- **Sections:** [Installation](https://rust-book.cs.brown.edu/ch01-01-installation.html); [Hello, World!](https://rust-book.cs.brown.edu/ch01-02-hello-world.html); [Hello, Cargo!](https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html).
- **Learning focus:** Install and verify Rust, compile a single source file with `rustc`, and create, build, check, and run a Cargo package.
- **Mastery evidence:** Create a package from scratch, identify `main`, distinguish compile from run, and explain the roles of `Cargo.toml`, debug builds, and release builds.
- **Built-in assessment:** Three embedded quizzes, one in each section.

### Unit 02 — Programming a Guessing Game

- **Source:** [Programming a Guessing Game](https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html)
- **Sections/flow:** Create a project; read user input; generate a random secret; compare guesses; handle invalid input; repeat with a loop.
- **Learning focus:** Experience variables, references, `Result`, `match`, external crates, type conversion, loops, and dependency use in one small program.
- **Mastery evidence:** Reconstruct the game in milestones and explain each compiler-guided refinement instead of copying the final listing.
- **Built-in assessment:** The chapter explicitly has no quizzes because it is a language tour and hands-on walkthrough.

### Unit 03 — Common Programming Concepts

- **Source:** [Common Programming Concepts](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)
- **Sections:** [Variables and Mutability](https://rust-book.cs.brown.edu/ch03-01-variables-and-mutability.html); [Data Types](https://rust-book.cs.brown.edu/ch03-02-data-types.html); [Functions](https://rust-book.cs.brown.edu/ch03-03-how-functions-work.html); [Comments](https://rust-book.cs.brown.edu/ch03-04-comments.html); [Control Flow](https://rust-book.cs.brown.edu/ch03-05-control-flow.html).
- **Learning focus:** Bindings, constants, shadowing, scalar and compound types, statements versus expressions, function signatures, branches, and loops.
- **Mastery evidence:** Choose appropriate types, trace expressions and control flow, and write small functions without relying on ownership concepts not yet introduced.
- **Built-in assessment:** Nine quiz embeds across variables, scalar and compound types, functions, and control flow.

### Unit 04 — Understanding Ownership

- **Source:** [Understanding Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html)
- **Sections:** [What Is Ownership?](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html); [References and Borrowing](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html); [Fixing Ownership Errors](https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html); [The Slice Type](https://rust-book.cs.brown.edu/ch04-04-slices.html); [Ownership Recap](https://rust-book.cs.brown.edu/ch04-05-ownership-recap.html).
- **Learning focus:** Stack and heap behavior, moves, cloning, copying, references, mutable borrowing, permissions, dangling-reference prevention, ownership-error repair, and slices.
- **Mastery evidence:** Predict move and borrow errors before compiling, repair errors by changing data flow rather than cloning automatically, and connect slices to source validity.
- **Built-in assessment:** Nine quiz embeds. This is the Brown edition's most substantially revised chapter and uses Aquascope-style ownership visualizations.

### Unit 05 — Using Structs to Structure Related Data

- **Source:** [Using Structs to Structure Related Data](https://rust-book.cs.brown.edu/ch05-00-structs.html)
- **Sections:** [Defining and Instantiating Structs](https://rust-book.cs.brown.edu/ch05-01-defining-structs.html); [An Example Program Using Structs](https://rust-book.cs.brown.edu/ch05-02-example-structs.html); [Methods](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html).
- **Learning focus:** Named fields, tuple and unit structs, field init shorthand, update syntax, derived traits, methods, associated functions, and multiple `impl` blocks.
- **Mastery evidence:** Turn loosely related values into a coherent domain type and choose correctly among `self`, `&self`, and `&mut self` receivers.
- **Built-in assessment:** Four quiz embeds across all three sections.

### Unit 06 — Enums and Pattern Matching

- **Source:** [Enums and Pattern Matching](https://rust-book.cs.brown.edu/ch06-00-enums.html)
- **Sections:** [Defining an Enum](https://rust-book.cs.brown.edu/ch06-01-defining-an-enum.html); [`match`](https://rust-book.cs.brown.edu/ch06-02-match.html); [`if let` and `let...else`](https://rust-book.cs.brown.edu/ch06-03-if-let.html); [Ownership Inventory #1](https://rust-book.cs.brown.edu/ch06-04-inventory.html).
- **Learning focus:** Variants with data, `Option`, exhaustive matching, patterns, catch-all arms, concise conditional destructuring, and ownership effects during matches.
- **Mastery evidence:** Model mutually exclusive states, handle every case, and predict when matching moves or borrows data.
- **Built-in assessment:** Three topic quizzes plus Ownership Inventory #1, which applies ownership reasoning to realistic scenarios.

### Unit 07 — Packages, Crates, and Modules

- **Source:** [Packages, Crates, and Modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- **Sections:** [Packages and Crates](https://rust-book.cs.brown.edu/ch07-01-packages-and-crates.html); [Modules and Privacy](https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html); [Paths](https://rust-book.cs.brown.edu/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html); [`use`](https://rust-book.cs.brown.edu/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html); [Separate Files](https://rust-book.cs.brown.edu/ch07-05-separating-modules-into-different-files.html).
- **Learning focus:** Packages, binary and library crates, module trees, visibility, absolute and relative paths, `use`, `as`, re-exporting, glob imports, and file layout.
- **Mastery evidence:** Restructure a working program without changing behavior and expose only a deliberate public API.
- **Built-in assessment:** Six quiz embeds spanning every section.

### Unit 08 — Common Collections

- **Source:** [Common Collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html)
- **Sections:** [Vectors](https://rust-book.cs.brown.edu/ch08-01-vectors.html); [Strings](https://rust-book.cs.brown.edu/ch08-02-strings.html); [Hash Maps](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html); [Ownership Inventory #2](https://rust-book.cs.brown.edu/ch08-04-inventory.html).
- **Learning focus:** Creating, updating, reading, and iterating vectors; UTF-8 string storage and slicing constraints; hash-map insertion, lookup, ownership, and entry updates.
- **Mastery evidence:** Select the correct collection, avoid invalid string indexing, and reason about references that collection mutation could invalidate.
- **Built-in assessment:** Five topic quiz embeds plus Ownership Inventory #2.

### Unit 09 — Error Handling

- **Source:** [Error Handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html)
- **Sections:** [`panic!`](https://rust-book.cs.brown.edu/ch09-01-unrecoverable-errors-with-panic.html); [`Result`](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html); [To `panic!` or Not](https://rust-book.cs.brown.edu/ch09-03-to-panic-or-not-to-panic.html).
- **Learning focus:** Unwinding versus aborting, backtraces, matching on `Result`, shortcuts such as `unwrap` and `expect`, propagation with `?`, and error-design guidelines.
- **Mastery evidence:** Classify failures as recoverable or invariant-breaking, preserve context, and design functions whose return types communicate failure.
- **Built-in assessment:** Four quiz embeds.

### Unit 10 — Generic Types, Traits, and Lifetimes

- **Source:** [Generic Types, Traits, and Lifetimes](https://rust-book.cs.brown.edu/ch10-00-generics.html)
- **Sections:** [Generic Data Types](https://rust-book.cs.brown.edu/ch10-01-syntax.html); [Traits](https://rust-book.cs.brown.edu/ch10-02-traits.html); [Lifetimes](https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html); [Ownership Inventory #3](https://rust-book.cs.brown.edu/ch10-04-inventory.html).
- **Learning focus:** Generic functions, structs and enums; trait definitions and bounds; `impl Trait`; conditional implementations; lifetime relationships and elision.
- **Mastery evidence:** Remove genuine duplication without over-generalizing, express required behavior with minimal bounds, and annotate returned-reference relationships correctly.
- **Built-in assessment:** Five topic quiz embeds plus Ownership Inventory #3.

### Unit 11 — Writing Automated Tests

- **Source:** [Writing Automated Tests](https://rust-book.cs.brown.edu/ch11-00-testing.html)
- **Sections:** [How to Write Tests](https://rust-book.cs.brown.edu/ch11-01-writing-tests.html); [Controlling Test Runs](https://rust-book.cs.brown.edu/ch11-02-running-tests.html); [Test Organization](https://rust-book.cs.brown.edu/ch11-03-test-organization.html).
- **Learning focus:** `#[test]`, assertions, custom messages, expected panics, `Result`-returning tests, filtering, ignored tests, unit tests, integration tests, and binary/library boundaries.
- **Mastery evidence:** Translate behavior into focused test cases and place tests according to whether they exercise internals or the public API.
- **Built-in assessment:** Three quiz embeds.

### Unit 12 — An I/O Project: Building a Command Line Program

- **Source:** [An I/O Project](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html)
- **Sections:** [CLI Arguments](https://rust-book.cs.brown.edu/ch12-01-accepting-command-line-arguments.html); [Reading a File](https://rust-book.cs.brown.edu/ch12-02-reading-a-file.html); [Modularity and Errors](https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html); [TDD](https://rust-book.cs.brown.edu/ch12-04-testing-the-librarys-functionality.html); [Environment Variables](https://rust-book.cs.brown.edu/ch12-05-working-with-environment-variables.html); [`stderr`](https://rust-book.cs.brown.edu/ch12-06-writing-to-stderr-instead-of-stdout.html).
- **Learning focus:** Integrate arguments, file reading, modular refactoring, test-driven functionality, environment configuration, and separation of normal output from errors.
- **Mastery evidence:** Build a `minigrep`-style program with a thin binary and testable library behavior.
- **Built-in assessment:** No embedded quizzes; the chapter is explicitly a hands-on recap project.

### Unit 13 — Functional Language Features: Iterators and Closures

- **Source:** [Functional Language Features](https://rust-book.cs.brown.edu/ch13-00-functional-features.html)
- **Sections:** [Closures](https://rust-book.cs.brown.edu/ch13-01-closures.html); [Iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html); [Improving the I/O Project](https://rust-book.cs.brown.edu/ch13-03-improving-our-io-project.html); [Performance](https://rust-book.cs.brown.edu/ch13-04-performance.html).
- **Learning focus:** Closure type inference and capture modes, `Fn` traits, iterator laziness, consuming and producing adapters, and zero-cost abstraction.
- **Mastery evidence:** Choose loops, iterator chains, or closures based on ownership and clarity, and explain when a closure borrows, mutably borrows, or moves captured data.
- **Built-in assessment:** Three quiz embeds across closures and iterators.

### Unit 14 — More About Cargo and Crates.io

- **Source:** [More About Cargo and Crates.io](https://rust-book.cs.brown.edu/ch14-00-more-about-cargo.html)
- **Sections:** [Release Profiles](https://rust-book.cs.brown.edu/ch14-01-release-profiles.html); [Publishing](https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html); [Workspaces](https://rust-book.cs.brown.edu/ch14-03-cargo-workspaces.html); [`cargo install`](https://rust-book.cs.brown.edu/ch14-04-installing-binaries.html); [Custom Commands](https://rust-book.cs.brown.edu/ch14-05-extending-cargo.html).
- **Learning focus:** Build profiles, documentation and public APIs, semantic versioning, publishing workflow, workspaces, binary installation, and Cargo extension conventions.
- **Mastery evidence:** Organize multiple related crates and prepare a small library as though it were a responsibly published package.
- **Built-in assessment:** Four quiz embeds across profiles, publishing, and workspaces.

### Unit 15 — Smart Pointers

- **Source:** [Smart Pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html)
- **Sections:** [`Box<T>`](https://rust-book.cs.brown.edu/ch15-01-box.html); [`Deref`](https://rust-book.cs.brown.edu/ch15-02-deref.html); [`Drop`](https://rust-book.cs.brown.edu/ch15-03-drop.html); [`Rc<T>`](https://rust-book.cs.brown.edu/ch15-04-rc.html); [`RefCell<T>`](https://rust-book.cs.brown.edu/ch15-05-interior-mutability.html); [Reference Cycles](https://rust-book.cs.brown.edu/ch15-06-reference-cycles.html).
- **Learning focus:** Heap indirection and recursive types, deref coercion, cleanup, shared ownership, interior mutability, runtime borrow enforcement, `Weak<T>`, and leaks.
- **Mastery evidence:** Choose the narrowest ownership tool, predict reference counts and runtime borrow failures, and prevent cycles with weak references.
- **Built-in assessment:** Six quiz embeds, one per section.

### Unit 16 — Fearless Concurrency

- **Source:** [Fearless Concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html)
- **Sections:** [Threads](https://rust-book.cs.brown.edu/ch16-01-threads.html); [Message Passing](https://rust-book.cs.brown.edu/ch16-02-message-passing.html); [Shared State](https://rust-book.cs.brown.edu/ch16-03-shared-state.html); [`Send` and `Sync`](https://rust-book.cs.brown.edu/ch16-04-extensible-concurrency-sync-and-send.html).
- **Learning focus:** Spawning and joining threads, `move` closures, channels, multiple producers, `Mutex<T>`, `Arc<T>`, deadlock awareness, and marker traits.
- **Mastery evidence:** Build both message-passing and shared-state versions of a task, explain ownership across threads, and guarantee shutdown.
- **Built-in assessment:** Four quiz embeds.

### Unit 17 — Fundamentals of Asynchronous Programming

- **Source:** [Async, Await, Futures, and Streams](https://rust-book.cs.brown.edu/ch17-00-async-await.html)
- **Sections:** [Futures and Syntax](https://rust-book.cs.brown.edu/ch17-01-futures-and-syntax.html); [Async Concurrency](https://rust-book.cs.brown.edu/ch17-02-concurrency-with-async.html); [Any Number of Futures](https://rust-book.cs.brown.edu/ch17-03-more-futures.html); [Streams](https://rust-book.cs.brown.edu/ch17-04-streams.html); [Async Traits](https://rust-book.cs.brown.edu/ch17-05-traits-for-async.html); [Futures, Tasks, and Threads](https://rust-book.cs.brown.edu/ch17-06-futures-tasks-threads.html).
- **Learning focus:** Creating and awaiting futures, concurrent composition, dynamic future collections, streams, pinning and async-related traits, and choosing tasks versus threads.
- **Mastery evidence:** Run independent I/O concurrently, bound or cancel work, consume streams, and explain why sequential `.await` is not concurrent.
- **Built-in assessment:** Five quiz embeds across the first five sections.

### Unit 18 — Object-Oriented Programming Features

- **Source:** [Object-Oriented Programming Features](https://rust-book.cs.brown.edu/ch18-00-oop.html)
- **Sections:** [OO Characteristics](https://rust-book.cs.brown.edu/ch18-01-what-is-oo.html); [Trait Objects](https://rust-book.cs.brown.edu/ch18-02-trait-objects.html); [OO Design Pattern](https://rust-book.cs.brown.edu/ch18-03-oo-design-patterns.html); [Ownership Inventory #4](https://rust-book.cs.brown.edu/ch18-04-inventory.html); [Design Trade-offs](https://rust-book.cs.brown.edu/ch18-05-design-challenge.html).
- **Learning focus:** Encapsulation, inheritance alternatives, dynamic dispatch with trait objects, state-pattern implementations, ownership in heterogeneous designs, and API trade-offs.
- **Mastery evidence:** Compare enums, generics, and trait objects for a changing design and justify the choice rather than merely making it compile.
- **Built-in assessment:** Three topic quizzes, Ownership Inventory #4, and four scenario-based design-tradeoff quizzes.

### Unit 19 — Patterns and Matching

- **Source:** [Patterns and Matching](https://rust-book.cs.brown.edu/ch19-00-patterns.html)
- **Sections:** [Pattern Locations](https://rust-book.cs.brown.edu/ch19-01-all-the-places-for-patterns.html); [Refutability](https://rust-book.cs.brown.edu/ch19-02-refutability.html); [Pattern Syntax](https://rust-book.cs.brown.edu/ch19-03-pattern-syntax.html).
- **Learning focus:** Patterns in `match`, `if let`, `while let`, `for`, `let`, and parameters; refutable versus irrefutable patterns; destructuring, guards, `@`, wildcards, and ignored values.
- **Mastery evidence:** Write precise destructuring patterns and explain why a pattern is valid or invalid in its syntactic position.
- **Built-in assessment:** Three quiz embeds.

### Unit 20 — Advanced Features

- **Source:** [Advanced Features](https://rust-book.cs.brown.edu/ch20-00-advanced-features.html)
- **Sections:** [Unsafe Rust](https://rust-book.cs.brown.edu/ch20-01-unsafe-rust.html); [Advanced Traits](https://rust-book.cs.brown.edu/ch20-02-advanced-traits.html); [Advanced Types](https://rust-book.cs.brown.edu/ch20-03-advanced-types.html); [Advanced Functions and Closures](https://rust-book.cs.brown.edu/ch20-04-advanced-functions-and-closures.html); [Macros](https://rust-book.cs.brown.edu/ch20-05-macros.html).
- **Learning focus:** Unsafe superpowers and safety contracts, associated types, operator overloading, fully qualified syntax, supertraits, newtypes, type aliases, never types, dynamically sized types, function pointers, returning closures, and macros.
- **Mastery evidence:** Encapsulate unsafe code behind a safe API, resolve advanced trait ambiguity, and select newtypes, aliases, functions, closures, or macros for the right reason.
- **Built-in assessment:** Five quiz embeds, one per section.

### Unit 21 — Final Project: Building a Multithreaded Web Server

- **Source:** [Final Project](https://rust-book.cs.brown.edu/ch21-00-final-project-a-web-server.html)
- **Sections:** [Single-Threaded Server](https://rust-book.cs.brown.edu/ch21-01-single-threaded.html); [Multithreaded Server](https://rust-book.cs.brown.edu/ch21-02-multithreaded.html); [Graceful Shutdown](https://rust-book.cs.brown.edu/ch21-03-graceful-shutdown-and-cleanup.html).
- **Learning focus:** TCP and basic HTTP, request parsing, response generation, thread-pool design, channels, job execution, cleanup, and graceful shutdown.
- **Mastery evidence:** Build the server incrementally, demonstrate parallel request handling, and prove workers terminate when the pool is dropped.
- **Built-in assessment:** No embedded quiz; the entire chapter is the capstone build and contains an invitation to implement thread-pool changes independently.

## Brown-specific assessment structure

The current source embeds interactive quizzes in Chapters 1, 3–11, and 13–20. Chapters 2 and 12 explicitly state that they omit quizzes because they are hands-on walkthroughs. Chapter 21 is a final project.

The Brown edition adds four cumulative **Ownership Inventories** after Chapters 6, 8, 10, and 18. Their stated purpose is to test ownership understanding in realistic scenarios inspired by common Rust questions.

Chapter 18 ends with a **Design Trade-offs** section containing scenario quizzes about references, trait-based trees, static versus dynamic dispatch, and intermediate representations. This is a model for asking learners to justify designs, not only predict compilation.

## Implications for the workbook rewrite

1. Use 22 chapter files: Introduction (`00`) plus Chapters `01`–`21`, or explicitly label the Introduction as the first orientation unit while preserving the book's numbering.
2. Keep each workbook chapter aligned to the source chapter's full section coverage, not merely its headline topic.
3. Treat Chapters 2, 12, and 21 as integration builds; their challenges should test reconstruction and extension of the walkthrough project.
4. Add cumulative ownership checks to workbook Chapters 6, 8, 10, and 18.
5. Make Chapter 18 include design-justification challenges comparing enums, generics, and trait objects.
6. For every project prerequisite, cite exact chapter or section links rather than generic topic labels.
7. Number tests inside each brief and distinguish unit tests for isolated rules from integration tests for external behavior and component boundaries.

## Source notes

- Live book landing page: <https://rust-book.cs.brown.edu/>
- Official source repository: <https://github.com/cognitive-engineering-lab/rust-book>
- Canonical table of contents: <https://github.com/cognitive-engineering-lab/rust-book/blob/main/src/SUMMARY.md>
- Interactive mechanics and Brown-specific changes: <https://rust-book.cs.brown.edu/>
- Quiz source directory used to verify current embed coverage: <https://github.com/cognitive-engineering-lab/rust-book/tree/main/quizzes>
