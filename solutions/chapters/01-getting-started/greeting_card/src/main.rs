/*
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
*/

fn main() {
    println!("================");
    println!("Welcome, Rustacean!");
    println!("================");
}
