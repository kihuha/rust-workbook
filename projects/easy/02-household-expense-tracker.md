# Easy 02 — Household expense tracker

## Project challenge

Build a small expense tracker that records purchases and summarizes spending by category. The challenge is to keep money exact while aggregating structured data.

## Required behavior

- Record a date, description, category, and positive amount in integer cents.
- List expenses in entry order.
- Report the total and totals grouped by category.
- Load and save a documented comma-separated format.
- Reject zero amounts and malformed rows with a line number.

## Example input

```text
2026-11-01,Bus,transport,150
2026-11-01,Lunch,food,800
2026-11-02,Groceries,food,4500
```

## Expected summary

```text
food: 5300 cents
transport: 150 cents
total: 5450 cents
```

## Rust skills combined

Structs, `Vec`, `HashMap`, string parsing, sorting, file I/O, `Result`, iterators, and tests.

## Done when

The totals are deterministic, a bad row identifies its source line, and a failed load never returns a partially valid ledger as if it succeeded.

## System-design foundation

Use integer minor units, define rounding explicitly, and reconcile every report to its source
records. This is the first version of the money invariants later required by the
[personal finance ledger](../medium/01-personal-finance-ledger.md) and PayForge.

### Foundation evidence

- `F01` — Property-test that every category/month summary equals the exact sum of its source records.
- `F02` — Test currency-scale and rounding boundaries without using binary floating point for money.

## Brown Rust Book prerequisites

Study [common collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [recoverable errors](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), [writing tests](https://rust-book.cs.brown.edu/ch11-01-writing-tests.html), [reading files](https://rust-book.cs.brown.edu/ch12-02-reading-a-file.html), and [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — The three example expenses total `5450` cents.
- `T02` — Food expenses total `5300` cents.
- `T03` — Zero cents and malformed amounts are rejected.
- `T04` — An empty ledger reports zero totals.

### Integration tests

- `T05` — Load the example file and verify the complete summary.
- `T06` — A malformed third row reports line `3` and returns no partial ledger.

## Stretch goals

Filter by date range or produce a monthly summary.
