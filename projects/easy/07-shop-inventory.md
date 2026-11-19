# Easy 07 — Shop inventory

## Project challenge

Build an inventory tool that tracks stock movements instead of directly replacing quantities. The challenge is to preserve valid state through every operation.

## Required behavior

- Store a SKU, item name, unit price in cents, and quantity.
- Support adding products, restocking, selling, listing, and low-stock reporting.
- Reject duplicate SKUs, zero movements, and sales above available stock.
- Calculate the total retail value with checked arithmetic.
- Persist inventory between runs.

## Example session

```text
$ inventory add BK-01 "Rust Book" 4500 3
$ inventory sell BK-01 2
Sold 2; remaining stock: 1
$ inventory value
Total retail value: 4500 cents
```

## Rust skills combined

Structs, `HashMap`, mutable borrowing, checked arithmetic, custom errors, command parsing, persistence, and tests.

## Done when

A rejected sale leaves quantity unchanged, totals cannot silently overflow, and stock operations are separate from CLI code.

## System-design foundation

Write the stock invariant before the implementation and test that rejected and repeated operations
are atomic. Then run generated command sequences. This directly prepares the
[event booking engine](../medium/09-event-booking-engine.md) and SwiftCart's reservation rules.

### Foundation evidence

- `F01` — Generate stock/add/sell/return sequences and assert quantity never becomes negative.
- `F02` — Repeat an operation ID and attempt an oversized sale; prove neither changes stock more than once.

## Brown Rust Book prerequisites

Study [methods](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html), [hash maps](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html), [panic policy](https://rust-book.cs.brown.edu/ch09-03-to-panic-or-not-to-panic.html), [automated tests](https://rust-book.cs.brown.edu/ch11-00-testing.html), and [CLI modularity](https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html).

## Suggested tests

### Unit tests

- `T01` — Selling two of three units leaves one.
- `T02` — Selling above stock returns an error and preserves quantity.
- `T03` — Duplicate SKUs and zero movements are rejected.
- `T04` — Retail-value overflow returns an error.

### Integration tests

- `T05` — Add, sell, save, reload, and verify the example product.
- `T06` — A failed CLI sale does not alter the saved inventory.

## Stretch goals

Record a movement history or import an opening inventory file.
