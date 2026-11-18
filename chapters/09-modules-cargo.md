# Chapter 9 — Modules and Cargo

## Challenge 025 — Split the banking library

**Mini brief:** The account and transaction code is growing. Restructure it into a small library with clear public and private boundaries.

**Practice:** Cargo package layout, modules, visibility, and `use` paths.

**Requirements:** Create modules for accounts and transactions, expose only the types and operations needed by the binary, and keep internal validation helpers private.

**Done when:** The program builds with `cargo check` and a reader can find each domain concept quickly.

**Stretch:** Add a module-level documentation comment explaining the library's purpose.

## Challenge 026 — Unit conversion crate

**Mini brief:** Create a reusable command-line converter for distance and temperature units.

**Practice:** Separating library logic from the CLI, Cargo targets, and reusable public functions.

**Requirements:** Put conversions in `src/lib.rs` and parsing/printing in `src/main.rs`. Support a small set of conversions and reject unknown units.

**Done when:** The library functions can be called independently of the command line.

**Stretch:** Add a second binary target for batch conversion from a text file.

## Challenge 027 — Dependency decision note

**Mini brief:** A text-processing feature could be written with the standard library or with a small third-party crate.

**Practice:** Cargo dependencies, version declarations, and evaluating dependency costs.

**Requirements:** Implement a simple word-frequency command using only the standard library. Then add one focused dependency for a meaningful improvement, document why it helps, and inspect the Cargo manifest and lockfile changes.

**Done when:** You can build both versions and explain what Cargo records in each file.

**Stretch:** Remove the dependency and compare code complexity and binary needs.
