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

## Stretch goals

Drop the lowest score in a category or generate per-student report files.
