# Medium 01 — Personal finance ledger

## Project challenge

Build a ledger that records accounts and immutable transactions, then derives balances and reports. The challenge is to make transaction history the source of truth instead of storing easily corrupted totals.

## Required behavior

- Create accounts with stable IDs and opening balances.
- Record deposits, withdrawals, and transfers in integer cents.
- Reject invalid amounts, missing accounts, and overdrafts.
- Generate account statements and category summaries for a date range.
- Save transactions atomically so a failed write does not replace valid data.

## Example flow

```text
checking opens with 100000
deposit salary 250000
transfer 80000 from checking to savings
withdraw rent 120000 from checking

checking balance: 150000 cents
savings balance: 80000 cents
```

## Architecture expectations

Separate domain types, ledger operations, persistence, report generation, and CLI parsing. Test domain logic with in-memory repositories or temporary files.

## Rust skills combined

Modules, enums, ownership, collections, iterators, custom errors, file replacement, traits, and unit plus integration tests.

## Done when

Every accepted transaction preserves total money except deposits and withdrawals, failed operations append nothing, and saved data survives a round trip.

## Brown Rust Book prerequisites

Study [public APIs](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [tests](https://rust-book.cs.brown.edu/ch11-00-testing.html), and [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — The example flow produces checking `150000` and savings `80000` cents.
- `T02` — An overdraft appends no transaction and preserves balances.
- `T03` — A transfer preserves the combined balance.

### Integration tests

- `T04` — Save, reload, and reproduce the same statement and balances.
- `T05` — Simulate a failed save and verify the previous ledger remains readable.

## Stretch goals

Add recurring transactions, CSV export, or budgets by category.
