# Chapter 10 — Closures and iterators

## Challenge 028 — Filtered score report

**Challenge:** Derive two related statistics from a score slice using iterator adapters and handle the case where no score passes.

**Your task:** Given a pass mark, compute the mean of passing scores and the number of scores below the mark. Do not use an indexing loop.

**Example input:** Scores `[72, 45, 88, 59, 90]`; pass mark `60`.

**Expected result:**

```text
Passing mean: 83.33
Needs support: 2
```

**Requirements:** A score equal to the pass mark passes. Return `None` for the mean when there are no passing scores. Empty input must return `None` and support count `0`.

**Done when:** `filter`, `map`, `sum`, or `count` express the calculation and no division by zero is possible.

**Stretch:** Implement the same behavior with a loop and compare clarity and number of passes over the data.

## Challenge 029 — Transaction pipeline

**Challenge:** Filter borrowed ledger entries, format matching rows, and total them without cloning the transactions.

**Your task:** Define a transaction with date, kind, description, and cents. Select deposits in an inclusive date range, produce display rows, and compute their total.

**Example input:**

```text
2026-01-02 deposit Salary 100000
2026-01-05 withdrawal Rent 40000
2026-01-10 deposit Gift 5000
range: 2026-01-01..=2026-01-07
```

**Expected result:**

```text
2026-01-02 | Salary | +100000
Total deposits: 100000 cents
```

**Requirements:** Filter the borrowed slice by kind and date. Keep the original ledger usable. Collect display strings only if the final API needs ownership.

**Done when:** Boundary dates are included, withdrawals are excluded, and no transaction is cloned.

**Stretch:** Extract named iterator helpers and compare their readability with one chained expression.

## Challenge 030 — Custom sequence iterator

**Challenge:** Implement iterator state and termination correctly for an inclusive page range.

**Your task:** Create `PageRange::new(start, end)` and implement `Iterator<Item = u32>`. Yield every page from `start` through `end` once.

**Example results:**

```text
PageRange::new(3, 6).collect() -> [3, 4, 5, 6]
PageRange::new(5, 5).collect() -> [5]
PageRange::new(6, 3).collect() -> []
```

**Requirements:** After returning `None`, later calls to `next` must also return `None`. Avoid overflow when `end == u32::MAX`.

**Done when:** The iterator works in `for`, with `collect`, and under repeated `next` calls after exhaustion.

**Stretch:** Implement an exact `size_hint` and explain its lower and upper bounds.
