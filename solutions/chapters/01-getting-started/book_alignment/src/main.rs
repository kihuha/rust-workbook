/*
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
*/

fn report() {
    println!("{}", "Rust Workbook");
    println!("{}", "Environment ready");
}

fn main() {
    report()
}
