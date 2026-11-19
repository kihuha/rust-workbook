# Easy 09 — Habit streak calculator

## Project challenge

Build a habit tracker that records completion dates and calculates streaks. The challenge is to handle ordered dates, duplicates, and gaps correctly.

## Required behavior

- Create habits with stable IDs and names.
- Mark a habit complete once per calendar date.
- Report total completions, current streak, and longest streak.
- List missing dates inside a requested range.
- Persist habits and completion dates.

## Example dates and result

```text
completions: 2026-11-01, 2026-11-02, 2026-11-04, 2026-11-05, 2026-11-06
as of: 2026-11-06
current streak: 3 days
longest streak: 3 days
missing: 2026-11-03
```

## Rust skills combined

Structs, sets, sorting, date handling, iterators, error handling, persistence, and tests.

## Done when

Duplicate completions do not increase counts, streaks work across month boundaries, and tests control the “as of” date instead of reading the real clock.

## Stretch goals

Add weekly targets or a text calendar view.
