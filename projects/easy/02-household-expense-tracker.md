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

## Stretch goals

Filter by date range or produce a monthly summary.
