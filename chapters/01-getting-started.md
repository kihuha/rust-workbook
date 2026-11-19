# Chapter 1 — Getting Started

## Book alignment

Study [Getting Started](https://rust-book.cs.brown.edu/ch01-00-getting-started.html): [Installation](https://rust-book.cs.brown.edu/ch01-01-installation), [Hello, World!](https://rust-book.cs.brown.edu/ch01-02-hello-world.html), and [Hello, Cargo!](https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html).

## Mastery goals

- Verify and update the Rust toolchain.
- Compile a single file with `rustc` and run its binary.
- Create, check, build, run, and format a Cargo package.

## Challenge 01.1 — Toolchain health report

### Brief

Create a program that prints a short development-environment report, then build it once with `rustc` and once as a Cargo package.

### Required behavior

- Print the application name and a message confirming the environment is ready.
- Record the installed `rustc` and `cargo` versions outside the program.
- Explain the source file and binary produced by `rustc`.
- Run `cargo check`, `cargo build`, and `cargo run` and explain their different roles.

### Expected output

```text
Rust Workbook
Environment ready
```

### Verification checklist

- Verify a `report()` helper returns both required lines.
- Verify the report contains no machine-specific path or version.


- Verify `rustc main.rs` produces a runnable binary with the expected output.
- Verify `cargo check` and `cargo run` succeed in the Cargo version.

### Done when

You can reproduce the program through both workflows and explain why Cargo is preferred as projects grow.

## Challenge 01.2 — Cargo greeting card

### Brief

Create a Cargo binary named `greeting_card` that prints a fixed three-line card. Practise the standard package layout and formatting workflow.

### Example output

```text
================
Welcome, Rustacean!
================
```

### Required behavior

- Keep executable code in `src/main.rs`.
- Create the package with Cargo rather than by hand.
- Run `cargo fmt --check` and `cargo check`.
- Inspect `Cargo.toml` and identify the package name, version, and edition.

### Verification checklist

- Verify a formatting helper returns exactly three lines.
- Verify the top and bottom borders have equal length.


- Verify the binary exits successfully.
- Verify standard output exactly matches the documented card.

### Done when

The package follows Cargo conventions and can be built from a clean checkout with one Cargo command.
