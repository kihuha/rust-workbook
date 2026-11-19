# Chapter 12 — An I/O Project: Building a Command-Line Program

## Book alignment

Study the [I/O Project](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html): CLI arguments, file reading, modular errors, TDD, environment variables, and writing errors to `stderr`.

## Mastery goals

- Keep the binary thin and the core behavior testable as a library.
- Parse configuration separately from executing work.
- Read files and propagate contextual errors.
- Separate normal output from diagnostic output.

## Challenge 12.1 — Build `logfind`

### Brief

Build a command-line tool that prints lines containing a query, following the chapter’s incremental project flow without copying `minigrep` directly.

### Required behavior

- Accept `query` and `file_path` positional arguments.
- Return matching lines with one-based line numbers.
- Read `LOGFIND_CASE_INSENSITIVE` to enable case-insensitive matching.
- Put search behavior in the library crate.
- Print failures to `stderr` and exit unsuccessfully.

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

## Challenge 12.2 — Extend `logfind` with context

### Brief

Add optional context lines while preserving testability and avoiding duplicate output when nearby matches overlap.

### Required behavior

- Accept `--before N` and `--after N`.
- Print each source line at most once.
- Mark matching lines with `>` and context lines with a space.
- Reject missing, negative, or malformed context counts.

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
