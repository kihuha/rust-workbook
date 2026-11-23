---
schema: 1
kind: chapter
chapter: 12
slug: io-command-line-project
title: "An I/O Project: Building a Command-Line Program"
---

# Chapter 12 — An I/O Project: Building a Command-Line Program

## Book alignment

Study the [I/O Project](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html): CLI arguments, file reading, modular errors, TDD, environment variables, and writing errors to `stderr`.

## Mastery goals

- Keep the binary thin and the core behavior testable as a library.
- Parse configuration separately from executing work.
- Read files and propagate contextual errors.
- Separate normal output from diagnostic output.

<!-- challenge
id: "12.1"
title: "Build `logfind`"
status: "not-started"
-->

## Challenge 12.1 — Build `logfind`

### Brief

Build a command-line tool that prints lines containing a query, following the chapter’s incremental project flow without copying `minigrep` directly.

### Required behavior

- Accept `query` and `file_path` positional arguments.
- Return matching lines with one-based line numbers.
- Read `LOGFIND_CASE_INSENSITIVE` to enable case-insensitive matching.
- Put search behavior in the library crate.
- Print failures to `stderr` and exit unsuccessfully.

### Expected library API and `main.rs` flow

```rust
pub struct Config {
    pub query: String,
    pub file_path: std::path::PathBuf,
    pub case_insensitive: bool,
}

pub fn parse_config(args: impl Iterator<Item = String>) -> Result<Config, String>;
pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<(usize, &'a str)>;
pub fn run(config: Config) -> Result<Vec<(usize, String)>, Box<dyn std::error::Error>>;
```

`main.rs` passes `std::env::args()` to `parse_config`, passes the result to `run`, and prints returned matches. It owns process-level error reporting and exit status. The library never calls `process::exit` and never prints search results.

### Example

```text
$ logfind rust notes.txt
2: Rust prevents many memory errors.
7: Learning Rust takes practice.
```

### Suggested tests

#### Unit tests

- `T01` — Verify case-sensitive search returns only exact-case matches.
- `T02` — Verify case-insensitive search returns `Rust` and `rust`.
- `T03` — Verify no match returns an empty result.
- `T04` — Verify reported line numbers start at one.

#### Integration tests

- `T05` — Verify a real temporary file produces the expected standard output.
- `T06` — Verify a missing file writes an error to `stderr`, not `stdout`.
- `T07` — Verify the environment variable changes matching behavior.

### Done when

The search library is independent of terminal I/O and the binary handles configuration, output, and process exit status.

<!-- /challenge -->

<!-- challenge
id: "12.2"
title: "Extend `logfind` with context"
status: "not-started"
-->

## Challenge 12.2 — Extend `logfind` with context

### Brief

Add optional context lines while preserving testability and avoiding duplicate output when nearby matches overlap.

### Required behavior

- Accept `--before N` and `--after N`.
- Print each source line at most once.
- Mark matching lines with `>` and context lines with a space.
- Reject missing, negative, or malformed context counts.

### Expected API changes and `main.rs` flow

Add `before: usize` and `after: usize` to `Config`. Represent each output row with its one-based line number, text, and whether it is a match.

```rust
pub fn search_with_context<'a>(
    query: &str,
    contents: &'a str,
    case_insensitive: bool,
    before: usize,
    after: usize,
) -> Vec<SearchLine<'a>>;
```

`main.rs` does not calculate context windows. It parses the new flags through `parse_config`, calls `run`, and prints each returned `SearchLine` with `>` for matches and a space for context.

### Example

```text
$ logfind error app.log --before 1 --after 1
  4: starting request
> 5: error: timeout
  6: retrying request
```

### Suggested tests

#### Unit tests

- `T01` — Verify zero context returns only matching lines.
- `T02` — Verify a match on the first line does not underflow.
- `T03` — Verify a match on the last line does not exceed input bounds.
- `T04` — Verify overlapping context windows do not duplicate lines.

#### Integration tests

- `T05` — Verify command-line flags produce the documented markers.
- `T06` — Verify malformed counts return a configuration error before reading the file.

### Done when

The new feature is driven by tests and does not move search policy back into `main.rs`.

<!-- /challenge -->
