# Chapter 13 — Functional Language Features: Iterators and Closures

## Book alignment

Study [Iterators and Closures](https://rust-book.cs.brown.edu/ch13-00-functional-features.html): closure capture, `Fn` traits, lazy iterators, adapters, improving the I/O project, and performance.

## Mastery goals

- Predict whether a closure borrows, mutably borrows, or moves captures.
- Build lazy iterator pipelines and know when they execute.
- Choose consuming and non-consuming adapters intentionally.
- Compare iterator clarity and performance with explicit loops.

## Challenge 13.1 — Configurable transaction report

### Brief

Filter and transform borrowed transactions using caller-provided closures.

### Required behavior

- Define transactions with description, category, and cents.
- Accept a predicate closure for inclusion.
- Accept a mapping closure for output formatting.
- Calculate the total without cloning transactions.
- Return `None` for the average of an empty selection.

### Example

```text
filter: category == "food" and cents >= 500
rows:
Lunch: 800 cents
Groceries: 4500 cents
total: 5300 cents
average: 2650.00 cents
```

### Suggested tests

#### Unit tests

- `T01` — Verify the predicate excludes non-food transactions.
- `T02` — Verify the mapper controls row formatting.
- `T03` — Verify total and average for two matches.
- `T04` — Verify no matches returns total zero and average `None`.

#### Integration tests

- `T05` — Verify the original transaction collection remains usable.
- `T06` — Verify a `move` closure can own a dynamically created filter value.

### Done when

The pipeline is lazy until consumed and closure bounds express how callbacks are used.

## Challenge 13.2 — Custom Fibonacci iterator

### Brief

Implement an iterator that generates Fibonacci numbers up to a caller-provided maximum value.

### Required behavior

- Implement `Iterator<Item = u64>` for `Fibonacci`.
- Yield `0, 1, 1, 2, ...` while values are at most the limit.
- Stop safely before addition overflow.
- Use adapters to select even values and sum a prefix.

### Example

```text
limit: 20
all: [0, 1, 1, 2, 3, 5, 8, 13]
even: [0, 2, 8]
even sum: 10
```

### Suggested tests

#### Unit tests

- `T01` — Verify limit `0` yields `[0]`.
- `T02` — Verify limit `1` yields `[0, 1, 1]`.
- `T03` — Verify limit `20` yields the documented sequence.
- `T04` — Verify calls after exhaustion continue returning `None`.

#### Integration tests

- `T05` — Verify `filter`, `take`, and `sum` work with the iterator.
- `T06` — Verify a near-`u64::MAX` limit does not panic from overflow.

### Done when

Iterator state, laziness, termination, and adapter composition can be explained from the implementation.
