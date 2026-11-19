# Medium 05 — Multithreaded file search

## Project challenge

Build a recursive search tool for text files. The challenge is to coordinate directory traversal and bounded parallel searching while producing stable, useful output.

## Required behavior

- Search a root directory recursively for a literal text pattern.
- Allow case-sensitive or case-insensitive matching.
- Skip binary files and configured directory names.
- Use a fixed number of workers rather than one thread per file.
- Print path, line number, and matching line in sorted path order.

## Example command and output

```text
$ rsearch --ignore-case ownership ./notes
notes/ch1.txt:12:Ownership controls when values are dropped.
notes/review.txt:4:Review ownership and borrowing.
```

## Rust skills combined

Paths, recursive traversal, threads, channels, byte and string handling, argument parsing, error context, sorting, and integration tests.

## Done when

Results match a single-threaded reference, unreadable files are reported without ending the search, workers are bounded, and output is deterministic.

## Brown Rust Book prerequisites

Study the [CLI and file project](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html), [iterator pipelines](https://rust-book.cs.brown.edu/ch13-02-iterators.html), and [threads plus channels](https://rust-book.cs.brown.edu/ch16-00-concurrency.html).

## Suggested tests

### Unit tests

- `T01` — Case-sensitive and insensitive matching follow their flags.
- `T02` — Binary detection skips documented binary input.
- `T03` — Excluded directory names are not traversed.

### Integration tests

- `T04` — Parallel results equal a sequential reference tree.
- `T05` — An unreadable file is reported while other files are searched.
- `T06` — Result order is identical across repeated runs.

## Stretch goals

Support regular expressions, contextual lines, or cancellation after a result limit.
