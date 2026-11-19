# Chapter 7 — Packages, Crates, and Modules

## Book alignment

Study [Packages, Crates, and Modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), including crates, privacy, paths, `use`, re-exports, and separate files.

## Mastery goals

- Distinguish packages, binary crates, library crates, and modules.
- Design a deliberate public API.
- Navigate module trees with absolute and relative paths.
- Split modules into files without changing their logical tree.

## Challenge 07.1 — Split a shop package

### Brief

Build one Cargo package containing a reusable inventory library and a thin command-line binary.

### Required behavior

- Create `inventory`, `pricing`, and `reports` modules in separate files.
- Keep stock mutation helpers private.
- Re-export the main `Product` and `Inventory` types from the crate root.
- Let `main.rs` use only the public API.

### Example output

```text
SKU BK-01 | Rust Book | quantity 3 | value 13500 cents
Inventory value: 13500 cents
```

### Verification checklist

- Verify private inventory logic updates quantity correctly from inside its module.
- Verify the report module calculates total value from public types.


- Verify a test in `tests/` can import the re-exported types.
- Verify external code cannot access private fields directly.
- Verify the binary produces the expected report through the public library API.

### Done when

The package has clear crate boundaries and rearranging files does not require exposing internals.

## Challenge 07.2 — Dual-binary converter package

### Brief

Create one library crate with two binaries: one converts a single value and another converts a batch file.

### Required behavior

- Put conversion rules in `src/lib.rs` or library modules.
- Provide `src/bin/convert.rs` and `src/bin/batch.rs`.
- Support Celsius/Fahrenheit and kilometers/miles.
- Use `use` paths that make call sites readable without glob imports.

### Example

```text
$ cargo run --bin convert -- 32 f c
0.00 c

$ cargo run --bin batch -- conversions.txt
line 1: 10 km = 6.21 mi
line 2: 32 f = 0.00 c
```

### Verification checklist

- Verify `32°F` converts to `0°C`.
- Verify `10 km` converts to approximately `6.21371 mi`.
- Verify unsupported unit pairs return an error.


- Verify both binary targets build.
- Verify the batch binary identifies an invalid input line.

### Done when

Both binaries reuse one library implementation and the module tree mirrors the domain rather than the executable layout.
