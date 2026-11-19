# Chapter 9 — Modules and Cargo

## Challenge 025 — Split the banking library

**Challenge:** Separate growing banking code into modules with a small public API and private implementation details.

**Your task:** Create a Cargo package with `accounts` and `transactions` library modules. The binary should create an account, deposit funds, withdraw funds, and print the balance through public APIs.

**Required layout:**

```text
src/
├── lib.rs
├── accounts.rs
├── transactions.rs
└── main.rs
```

**Example flow and output:**

```text
open account with 10000 cents
deposit 2500 cents
withdraw 3000 cents
-> Balance: 9500 cents
```

**Requirements:** Keep field mutation and validation helpers private. Export only the types and operations needed by `main`. Use clear `use` paths.

**Done when:** `cargo check` succeeds and direct access to private account fields from `main` fails to compile.

**Stretch:** Add module-level `//!` documentation and confirm it appears in `cargo doc --open`.

## Challenge 026 — Unit conversion crate

**Challenge:** Keep reusable conversion rules independent from command-line parsing and printing.

**Your task:** Put conversion functions in `src/lib.rs` and the CLI in `src/main.rs`. Support kilometers to miles, miles to kilometers, Celsius to Fahrenheit, and Fahrenheit to Celsius.

**Example commands and output:**

```text
cargo run -- 10 km mi  -> 6.21 mi
cargo run -- 32 f c    -> 0.00 c
cargo run -- 5 kg mi   -> error: unsupported unit kg
```

Use `1 km = 0.621371 mi`, `F = C × 9/5 + 32`, and `C = (F - 32) × 5/9`.

**Requirements:** Library functions must not read arguments or print. CLI parsing must reject missing values, invalid numbers, and unsupported conversion pairs.

**Done when:** Library functions have direct tests and work without the binary.

**Stretch:** Add `src/bin/batch.rs` to convert requests from a text file.

## Challenge 027 — Dependency decision note

**Challenge:** Evaluate a dependency by comparing a working standard-library version with a focused crate-based improvement.

**Your task:** Build a word-frequency CLI using only `std`. Then add one small dependency that improves tokenization, Unicode handling, CLI parsing, or error reporting.

**Example input and base output:**

```text
input: "Rust rust, safe!"
rust: 2
safe: 1
```

**Requirements:** Record the chosen crate and exact improvement. Show the relevant `Cargo.toml` change and explain why `Cargo.lock` changed. Keep both versions comparable through tests or commits.

**Done when:** Both versions build and your note explains the benefit, maintenance cost, transitive dependencies, and whether you would keep the crate.

**Stretch:** Remove the dependency cleanly and confirm the manifest, lockfile, imports, and code still build.
