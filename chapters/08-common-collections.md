---
schema: 1
kind: chapter
chapter: 8
slug: common-collections
title: "Common Collections"
---

# Chapter 8 — Common Collections

## Book alignment

Study [Common Collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html): vectors, UTF-8 strings, hash maps, and [Ownership Inventory #2](https://rust-book.cs.brown.edu/ch08-04-inventory.html).

## Mastery goals

- Select a vector, string, or hash map for the data relationship.
- Update collections without invalidating active references.
- Treat UTF-8 text safely instead of indexing strings as characters.
- Use entry APIs for conditional map updates.

<!-- challenge
id: "08.1"
title: "Word concordance"
status: "not-started"
-->

## Challenge 08.1 — Word concordance

### Brief

Build a concordance that maps each normalized word to the line numbers on which it appears.

### Required behavior

- Normalize words to lowercase and remove surrounding punctuation.
- Store unique line numbers per word in ascending order.
- Print words alphabetically.
- Preserve apostrophes inside words.

### Expected functions and `main.rs` flow

```rust
fn normalize_word(token: &str) -> Option<String>;
fn build_concordance(text: &str) -> std::collections::HashMap<String, Vec<usize>>;
fn format_concordance(concordance: &std::collections::HashMap<String, Vec<usize>>) -> String;
```

`main` reads the input file into a string, calls `build_concordance`, then prints `format_concordance`. Sorting belongs in formatting because `HashMap` iteration order is not alphabetical.

### Example input

```text
Rust is fast.
Rust is memory-safe.
Fast tools matter.
```

### Expected output

```text
fast: 1, 3
is: 1, 2
memory-safe: 2
rust: 1, 2
tools: 3
```

### Verification checklist

- Verify mixed-case `Rust` and `rust` share one key.
- Verify repeated words on one line store that line only once.
- Verify surrounding punctuation is removed.
- Verify empty input produces an empty map.


- Verify the complete report is alphabetically ordered.
- Verify UTF-8 text is processed without byte-index panics.

### Done when

The implementation uses collection APIs deliberately and never indexes a `String` by integer position.

<!-- /challenge -->

<!-- challenge
id: "08.2"
title: "Ownership Inventory: mutable collection references"
status: "not-started"
-->

## Challenge 08.2 — Ownership Inventory: mutable collection references

### Brief

Explore when vector and map references become invalid by implementing a safe roster update workflow and documenting rejected alternatives.

### Required behavior

- Store student names in a vector and scores in a hash map.
- Read a student reference, use it, then mutate the vector only after its last use.
- Update scores through `entry` without holding a conflicting map reference.
- Include commented compile-fail examples for invalid reference use.

### Expected functions and `main.rs` flow

```rust
fn add_score(scores: &mut std::collections::HashMap<String, u32>, name: &str, points: u32);
fn add_student(roster: &mut Vec<String>, name: String);
```

`main` reads and prints the selected roster entry before calling `add_student`, so no element reference crosses the vector mutation. It calls `add_score` without retaining another reference into the map, then prints the updated score and roster.

### Example

```text
selected: Amina
score before: 7
score after: 9
roster after addition: ["Amina", "Kamau", "Zuri"]
```

### Verification checklist

- Verify adding two points changes Amina’s score from `7` to `9`.
- Verify a missing student is inserted with the documented initial score.
- Verify roster order is preserved after appending.


- Add a compile-fail example that pushes while a vector element reference is used later.
- Verify the corrected workflow prints both pre-update and post-update values.

### Done when

You can explain why collection mutation may relocate data and how last use affects borrow validity.

<!-- /challenge -->
