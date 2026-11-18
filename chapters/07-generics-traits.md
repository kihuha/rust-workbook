# Chapter 7 — Generics and traits

## Challenge 019 — Reusable bounds checker

**Mini brief:** A sensor dashboard needs a helper that checks whether a value falls within an inclusive range.

**Practice:** Generic functions and trait bounds.

**Requirements:** Write a generic `within` helper for ordered, copyable values. Use it with integers and floating-point values. Keep the bound as small as the implementation permits.

**Done when:** Both types work and you can explain why the chosen traits are needed.

**Stretch:** Add a test with a custom type that implements the required ordering traits.

## Challenge 020 — Printable report items

**Mini brief:** A reporting tool accepts several kinds of values and produces a consistent one-line description for each.

**Practice:** Defining a trait, implementing it for multiple structs, and using trait bounds.

**Requirements:** Define a `ReportLine` trait and implement it for a `Deposit` and a `Withdrawal`. Write a generic function that prints any reportable value.

**Done when:** The shared function works for both types without knowing their concrete fields.

**Stretch:** Add a generic vector printer that accepts a slice of any `ReportLine` type.

## Challenge 021 — Configurable statistics

**Mini brief:** A data tool computes a statistic over a list of values; callers choose the rule used to combine them.

**Practice:** Generic types and trait-based abstractions, with a focus on useful API boundaries.

**Requirements:** Create a small `Metric` trait with an operation for combining two values. Implement sum and maximum metrics for integer input. Make a function consume a slice and return an optional result for empty input.

**Done when:** The algorithm is shared and each metric defines only its combining behavior.

**Stretch:** Compare the trait approach with a closure parameter after studying closures.
