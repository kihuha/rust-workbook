# Easy 03 — Text statistics reporter

## Project challenge

Build a program that reads a text file and produces a useful language report. The challenge is to normalize text consistently and sort computed results predictably.

## Required behavior

- Report line, word, and non-whitespace character counts.
- Normalize words to lowercase and remove surrounding punctuation.
- Print the five most frequent words.
- Break frequency ties alphabetically.
- Return a clear error when the file cannot be read.

## Example input

```text
Rust is fast.
Rust is safe!
```

## Expected report

```text
lines: 2
words: 6
non-whitespace characters: 24
is: 2
rust: 2
fast: 1
safe: 1
```

## Rust skills combined

Strings and slices, iterators, `HashMap`, sorting, file I/O, ownership, error propagation, and tests.

## Done when

Empty files work, output ordering is stable, and tokenization behavior is documented with tests for punctuation and mixed case.

## System-design foundation

Separate streaming ingestion, aggregation, and deterministic presentation. Record memory growth as
input grows and identify which statistics require retaining full input. These are the same choices
made by log, search, and metrics pipelines.

### Foundation evidence

- `F01` — Feed input through a reader that returns very small chunks and prove results match whole-file input.
- `F02` — Record peak memory and elapsed time for 1 MiB, 10 MiB, and 100 MiB generated inputs and explain the growth.

## Brown Rust Book prerequisites

Study [slices](https://rust-book.cs.brown.edu/ch04-04-slices.html), [UTF-8 strings](https://rust-book.cs.brown.edu/ch08-02-strings.html), [hash maps](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html), [file errors](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), and [iterator processing](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — The example contains two lines and six words.
- `T02` — `Rust` and `rust` count under one normalized key.
- `T03` — Equal frequencies sort alphabetically.
- `T04` — Empty text produces zero counts and no top words.

### Integration tests

- `T05` — Reading the example file prints the documented report.
- `T06` — A missing file produces an error and no partial report.

## Stretch goals

Add minimum word length, excluded-word, or top-N command-line options.
