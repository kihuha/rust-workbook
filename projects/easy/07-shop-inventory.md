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

## Stretch goals

Record a movement history or import an opening inventory file.
