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

## System-design foundation

Inject the clock and define timezone/day-boundary behavior instead of reading ambient time inside
domain logic. Deterministic time is required later for leases, retries, retention, quotas, and
distributed failure tests.

### Foundation evidence

- `F01` — Run the same cases under clocks straddling midnight, month/year boundaries, leap day, and the chosen timezone policy.
- `F02` — Shuffle and duplicate input records and prove the documented normalization yields deterministic streaks.

## Brown Rust Book prerequisites

Study [domain structs](https://rust-book.cs.brown.edu/ch05-00-structs.html), [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [recoverable errors](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), [test organization](https://rust-book.cs.brown.edu/ch11-03-test-organization.html), and [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — The example dates produce a current and longest streak of three.
- `T02` — Duplicate completion dates do not increase totals.
- `T03` — A streak continues correctly across a month boundary.
- `T04` — No completion on the as-of date produces a current streak of zero.

### Integration tests

- `T05` — Save and reload completions without losing date order.
- `T06` — The missing-date report identifies `2026-11-03` in the example range.

## Stretch goals

Add weekly targets or a text calendar view.
