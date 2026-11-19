# Easy 06 — Quiz game

## Project challenge

Build a terminal quiz that loads questions from a file, checks answers, and reports a final score. The challenge is to model different question types behind one game flow.

## Required behavior

- Support multiple-choice and true-or-false questions.
- Reject malformed question files before the game begins.
- Normalize user answers without changing displayed question text.
- Give immediate correct or incorrect feedback.
- Report the score and percentage at the end.

## Example interaction

```text
1. Which keyword creates an immutable variable by default?
A) let  B) mut  C) var
> a
Correct!

Score: 1/1 (100%)
```

## Rust skills combined

Enums, structs, pattern matching, buffered input, parsing, collections, error handling, modules, and tests.

## Done when

Both question types use one quiz loop, malformed answer keys produce useful errors, and scoring logic is tested without terminal input.

## Brown Rust Book prerequisites

Study [enums and `match`](https://rust-book.cs.brown.edu/ch06-00-enums.html), [modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [`Result`](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), and [test organization](https://rust-book.cs.brown.edu/ch11-03-test-organization.html).

## Suggested tests

### Unit tests

- `T01` — `a` and `A` both match answer `A`.
- `T02` — A wrong answer increments attempts but not score.
- `T03` — Both question variants use the same scoring function.
- `T04` — A malformed answer key is rejected before play.

### Integration tests

- `T05` — The example interaction ends with `1/1 (100%)`.
- `T06` — Loading a mixed question file preserves question order.

## Stretch goals

Shuffle questions, limit attempts, or save high scores.
