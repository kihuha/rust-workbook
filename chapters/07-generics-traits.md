# Chapter 7 — Generics and traits

## Challenge 019 — Reusable bounds checker

**Challenge:** One range-checking algorithm should work for several ordered types without requiring traits it does not use.

**Your task:** Implement `within<T>(value: T, min: T, max: T) -> bool`. The range is inclusive.

**Example calls:**

```text
within(7, 1, 10)       -> true
within(10, 1, 10)      -> true
within(12, 1, 10)      -> false
within(3.5, 2.0, 4.0)  -> true
```

**Requirements:** Choose the smallest useful trait bounds. If your implementation does not copy values, do not add `Copy` merely for convenience. Document behavior when `min > max`.

**Done when:** Integer and floating-point examples work and every trait bound can be justified from an operation in the function body.

**Stretch:** Define a custom ordered type and use it with `within`.

## Challenge 020 — Printable report items

**Challenge:** Deposits and withdrawals contain different data but must participate in one reporting API.

**Your task:** Define `ReportLine` with `report_line(&self) -> String`. Implement it for `Deposit` and `Withdrawal`, then write a generic `print_report<T: ReportLine>` function.

**Example values and output:**

```text
Deposit { source: "Salary", cents: 250000 }
-> "Deposit from Salary: +250000 cents"

Withdrawal { category: "Rent", cents: 80000 }
-> "Withdrawal for Rent: -80000 cents"
```

**Requirements:** The generic printer may use only the trait API, not the concrete fields. Both implementations must borrow their value.

**Done when:** The same print function accepts both types and produces the expected lines.

**Stretch:** Write a generic function that prints a slice containing one concrete `ReportLine` type, then compare it with a heterogeneous `Vec<Box<dyn ReportLine>>`.

## Challenge 021 — Configurable statistics

**Challenge:** Share the traversal of a list while allowing callers to choose how values are combined.

**Your task:** Define a `Metric` trait with a combine operation. Implement `Sum` and `Maximum` for `i32`, then write one function that evaluates any metric over a slice.

**Example results:**

```text
values [3, 8, 2, 5], Sum     -> Some(18)
values [3, 8, 2, 5], Maximum -> Some(8)
values [], any metric         -> None
```

**Requirements:** The shared algorithm must not contain metric-specific branches. Each metric implementation should define only its combination rule.

**Done when:** Both metrics reuse the same traversal and empty input has an explicit result.

**Stretch:** Replace the trait value with a closure parameter and compare the call sites.
