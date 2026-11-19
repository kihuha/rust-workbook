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

## Stretch goals

Add phrase search, term-frequency scoring, or file watching.
