# Easy 08 — Reading progress tracker

## Project challenge

Build a tracker for books and reading progress. The challenge is to model state transitions and calculate progress without allowing impossible page values.

## Required behavior

- Store a stable ID, title, author, total pages, current page, and status.
- Use statuses such as `NotStarted`, `Reading`, and `Finished`.
- Update the current page and derive the correct status.
- List books by status and show completion percentage.
- Save and load the collection.

## Example result

```text
1 | The Rust Book | 120/500 pages | Reading | 24.0%
2 | Rust in Action | 400/400 pages | Finished | 100.0%
```

**Invalid update:** Setting book `1` to page `700` must return an error and preserve page `120`.

## Rust skills combined

Structs, enums, methods, validation, collections, filtering, formatting, persistence, and tests.

## Done when

Status always agrees with page progress, zero-page books are rejected, and invalid updates never partially mutate a book.

## Brown Rust Book prerequisites

Study [structs](https://rust-book.cs.brown.edu/ch05-00-structs.html), [enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [tests](https://rust-book.cs.brown.edu/ch11-00-testing.html), and [filtering with iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — Page zero produces `NotStarted`.
- `T02` — Page `120` of `500` produces `Reading` and `24%`.
- `T03` — Page `400` of `400` produces `Finished`.
- `T04` — Page `700` of `500` is rejected without changing progress.

### Integration tests

- `T05` — Save and reload books while preserving derived status.
- `T06` — Filtering by `Finished` returns only completed books.

## Stretch goals

Add reading sessions and estimate completion from average pages per day.
