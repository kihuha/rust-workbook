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

## Stretch goals

Add minimum word length, excluded-word, or top-N command-line options.
