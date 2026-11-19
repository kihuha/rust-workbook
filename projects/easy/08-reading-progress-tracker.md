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

## Stretch goals

Add reading sessions and estimate completion from average pages per day.
