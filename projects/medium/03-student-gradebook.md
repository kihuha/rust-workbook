# Medium 03 — Student gradebook

## Project challenge

Build a gradebook that imports assessment data and applies configurable grading rules. The challenge is to keep raw scores separate from policy-driven final grades.

## Required behavior

- Store students, assessments, maximum scores, and weights.
- Import scores from CSV and identify row-level errors.
- Validate that assessment weights total 100%.
- Calculate weighted totals and letter grades from a configurable scale.
- Report missing scores separately from numeric zeroes.

## Example calculation

```text
Homework: 80/100 at 40% -> 32.0
Exam: 70/100 at 60%     -> 42.0
Final: 74.0%            -> C
```

## Architecture expectations

Keep CSV decoding, validation, grade policy, and report formatting in separate modules. Represent missing scores explicitly with `Option` or a domain enum.

## Rust skills combined

Structs, maps, decimal or carefully tested floating-point calculations, parsing, iterators, traits, custom errors, modules, and tests.

## Done when

Invalid policies fail before grading, missing scores follow a documented rule, imports are all-or-nothing, and example calculations are covered by tests.

## Brown Rust Book prerequisites

Study [`Option` and matching](https://rust-book.cs.brown.edu/ch06-00-enums.html), [modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [errors](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), and [tests](https://rust-book.cs.brown.edu/ch11-00-testing.html).

## Suggested tests

### Unit tests

- `T01` — The example weighted score is `74.0%` and grade `C`.
- `T02` — Policies whose weights do not total `100%` are rejected.
- `T03` — A missing score is distinguishable from score zero.
- `T04` — Grade boundaries follow the configured scale exactly.

### Integration tests

- `T05` — Import a valid CSV and produce a report for every student.
- `T06` — One invalid row rejects the whole import with its line number.

## Stretch goals

Drop the lowest score in a category or generate per-student report files.
