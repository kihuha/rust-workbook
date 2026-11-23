---
schema: 1
kind: chapter
chapter: 4
slug: understanding-ownership
title: "Understanding Ownership"
---

# Chapter 4 — Understanding Ownership

## Book alignment

Study [Understanding Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html): ownership, references and borrowing, fixing ownership errors, slices, and the ownership recap.

## Mastery goals

- Trace moves, copies, clones, and drops.
- Choose owned values, shared references, or mutable references intentionally.
- Keep references valid and prevent conflicting access.
- Return slices that borrow from valid source data.

<!-- challenge
id: "04.1"
title: "Document handoff"
status: "not-started"
-->

## Challenge 04.1 — Document handoff

### Brief

Model a document that is inspected, renamed, and archived without unnecessary cloning. Annotate where ownership moves and where values are borrowed.

### Required behavior

- Store a title and body as owned strings.
- `word_count` must borrow the document.
- `rename` must mutate through `&mut`.
- `archive` must consume the document and return an archive record.
- The caller must not use the document after archiving.

### Expected interface and `main.rs` flow

```rust
fn word_count(document: &Document) -> usize;
fn rename(document: &mut Document, new_title: String);
fn archive(document: Document) -> ArchiveRecord;
```

`main` creates one `Document`, borrows it for `word_count`, mutably borrows it for `rename`, then moves it into `archive`. It prints each result in the example order. Do not clone the document to keep using it after `archive`.

### Example

```text
title before: Draft
words: 4
title after: Rust Notes
archived: Rust Notes (4 words)
```

### Verification checklist

- Verify counting words does not consume or mutate the document.
- Verify renaming changes only the title.
- Verify archiving preserves the final title and word count.


- Verify the full inspect, rename, and archive flow prints expected values.
- Add a compile-fail example showing that an archived document cannot be reused.

### Done when

Each function signature communicates its ownership intent and no clone is used without a written reason.

<!-- /challenge -->

<!-- challenge
id: "04.2"
title: "Borrowed command parser"
status: "not-started"
-->

## Challenge 04.2 — Borrowed command parser

### Brief

Parse a command line into borrowed slices so the parser does not allocate copies of the command or argument.

### Required behavior

- Implement `parse_command(input: &str) -> Option<(&str, &str)>`.
- Trim surrounding whitespace.
- Return the first word as the command and the remaining trimmed text as its argument.
- Return `None` for blank input.

### Expected `main.rs` flow

`main` joins its command-line arguments into one input string, calls `parse_command(&input)`, and prints the borrowed command and argument. `parse_command` performs no printing and returns slices of `input` rather than new `String` values.

### Example cases

```text
"add Learn Rust" -> Some(("add", "Learn Rust"))
"list"           -> Some(("list", ""))
"   "            -> None
```

### Verification checklist

- Verify a command and multiword argument are split correctly.
- Verify a command without an argument returns an empty argument slice.
- Verify surrounding whitespace is ignored.
- Verify blank input returns `None`.


- Verify parsed slices remain usable while the source string is alive.
- Add a compile-fail example showing a result cannot outlive its source.

### Done when

The result borrows from the input, contains no cloned strings, and its validity can be explained from the function signature.

<!-- /challenge -->
