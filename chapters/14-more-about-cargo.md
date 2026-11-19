# Chapter 14 — More About Cargo and Crates.io

## Book alignment

Study [More About Cargo](https://rust-book.cs.brown.edu/ch14-00-more-about-cargo.html): release profiles, documentation and publishing, workspaces, `cargo install`, and custom Cargo commands.

## Mastery goals

- Tune development and release profiles deliberately.
- Document and package a public library responsibly.
- Organize related packages in a workspace.
- Understand installation and Cargo subcommand conventions.

## Challenge 14.1 — Publish-ready units library

### Brief

Prepare a small conversion library as though it will be published, without actually publishing it.

### Required behavior

- Give the package complete metadata and a deliberate version.
- Add crate-level and public-item documentation with examples.
- Configure a release profile and explain the chosen settings.
- Run formatting, Clippy, tests, doctests, and `cargo package --list`.
- Inspect the package contents before packaging.

### Example public API

```rust
let miles = units::kilometers_to_miles(10.0);
assert!((miles - 6.21371).abs() < 0.00001);
```

### Suggested tests

#### Unit tests

- `T01` — Verify zero converts to zero.
- `T02` — Verify a known conversion within documented tolerance.
- `T03` — Verify inverse conversions approximately round-trip.

#### Integration tests

- `T04` — Verify external tests can use only the documented public API.
- `T05` — Verify every documentation example compiles as a doctest.
- `T06` — Verify `cargo package --list` excludes temporary and secret files.

### Done when

The crate can be understood from generated documentation and packaged reproducibly without being uploaded.

## Challenge 14.2 — Workspace calculator suite

### Brief

Create a workspace containing a core library, a CLI package, and a separate report package.

### Required behavior

- Define a virtual workspace root.
- Share one internal library through path dependencies.
- Keep package-specific dependencies in their owning package.
- Run all tests from the workspace root.
- Document how a binary could be installed with `cargo install --path`.

### Example layout

```text
calculator-workspace/
├── Cargo.toml
├── calc-core/
├── calc-cli/
└── calc-report/
```

### Suggested tests

#### Unit tests

- `T01` — Verify core arithmetic independently of both consumers.
- `T02` — Verify report formatting in its own package.

#### Integration tests

- `T03` — Verify `cargo test --workspace` passes from the root.
- `T04` — Verify both consumer packages resolve the same core package.
- `T05` — Verify `cargo install --path calc-cli` produces the expected binary.

### Done when

Package boundaries are purposeful and the workspace coordinates builds without merging all code into one crate.
