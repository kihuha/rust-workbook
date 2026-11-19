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

## Stretch goals

Shuffle questions, limit attempts, or save high scores.
