# Chapter 4 — Understanding Ownership

## Book alignment

Study [Understanding Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html): ownership, references and borrowing, fixing ownership errors, slices, and the ownership recap.

## Mastery goals

- Trace moves, copies, clones, and drops.
- Choose owned values, shared references, or mutable references intentionally.
- Keep references valid and prevent conflicting access.
- Return slices that borrow from valid source data.

## Challenge 04.1 — Document handoff

### Brief

Model a document that is inspected, renamed, and archived without unnecessary cloning. Annotate where ownership moves and where values are borrowed.

### Required behavior

- Store a title and body as owned strings.
- `word_count` must borrow the document.
- `rename` must mutate through `&mut`.
- `archive` must consume the document and return an archive record.
- The caller must not use the document after archiving.

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

## Challenge 04.2 — Borrowed command parser

### Brief

Parse a command line into borrowed slices so the parser does not allocate copies of the command or argument.

### Required behavior

- Implement `parse_command(input: &str) -> Option<(&str, &str)>`.
- Trim surrounding whitespace.
- Return the first word as the command and the remaining trimmed text as its argument.
- Return `None` for blank input.

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
