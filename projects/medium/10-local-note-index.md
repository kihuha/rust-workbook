# Medium 10 — Local note index

## Project challenge

Build a local search index for Markdown notes. The challenge is to keep an index synchronized with changing files and return ranked results with useful context.

## Required behavior

- Scan a note directory for Markdown files.
- Extract titles, headings, tags, and normalized terms.
- Build an inverted index from terms to documents.
- Search one or more terms and rank notes by match count.
- Update only files whose path, modification time, or content hash changed.

## Example query

```text
$ notes search "ownership borrowing"
1. rust-basics.md (2 terms, 7 matches)
   ...ownership determines who drops a value...
2. review.md (1 term, 2 matches)
```

## Rust skills combined

Filesystem metadata, maps and sets, text parsing, serialization, incremental updates, iterators, error handling, modules, and integration tests.

## Done when

Deleted notes disappear from the index, unchanged notes are not reprocessed, result order is deterministic, and corrupt index data triggers a safe rebuild.

## System-design foundation

Separate authoritative documents from rebuildable index segments, version the analyzer, and publish
a rebuilt generation atomically. Measure indexing and query latency as the corpus grows. Continue
with [SearchGrid](../../real-world-usage/13-search-observability/02-searchgrid/README.md).

### Foundation evidence

- `F01` — Crash/fail before and after generation publication; startup exposes either the old or new complete index, never a mixture.
- `F02` — Benchmark index and representative queries at 1,000, 10,000, and 100,000 generated notes; report latency, memory, and index bytes.

## Brown Rust Book prerequisites

Study [strings, maps, and sets](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [error recovery](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [reading files](https://rust-book.cs.brown.edu/ch12-02-reading-a-file.html), and [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — Extract title, headings, tags, and terms from one note.
- `T02` — Rank a two-term match above a one-term match.
- `T03` — Equal-ranked notes sort deterministically.

### Integration tests

- `T04` — Unchanged notes are skipped on the second index run.
- `T05` — Modified and deleted files update the index correctly.
- `T06` — Corrupt persisted index data triggers a complete safe rebuild.

## Stretch goals

Add phrase search, term-frequency scoring, or file watching.
