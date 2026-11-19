# Chapter 10 — Generic Types, Traits, and Lifetimes

## Book alignment

Study [Generic Types, Traits, and Lifetimes](https://rust-book.cs.brown.edu/ch10-00-generics.html): generic data types, traits, lifetime relationships, and [Ownership Inventory #3](https://rust-book.cs.brown.edu/ch10-04-inventory.html).

## Mastery goals

- Generalize genuine duplication across types.
- Express required behavior with focused trait bounds.
- Choose static dispatch or `impl Trait` deliberately.
- Annotate relationships between borrowed inputs and outputs.

## Challenge 10.1 — Generic measurement range

### Brief

Create a generic inclusive range type and a trait-based reporting API for sensor measurements.

### Required behavior

- Define `Range<T>` with `min` and `max`.
- Implement `contains` for ordered values.
- Define a `LabeledMeasurement` trait returning a label and report text.
- Write one report function that accepts any implementing type.

### Example

```text
temperature: 24.5
allowed: 18.0..=26.0
status: within range
```

### Verification checklist

- Verify both endpoints are included for integer ranges.
- Verify `24.5` is inside `18.0..=26.0`.
- Verify a value below the minimum is excluded.
- Verify the generic reporter works for two measurement structs.


- Verify a report can be generated without exposing concrete fields.
- Verify every trait bound corresponds to an operation used by the implementation.

### Done when

The abstraction removes real duplication and does not require unnecessary bounds such as `Copy` or `Display`.

## Challenge 10.2 — Ownership Inventory: borrowed search results

### Brief

Search borrowed catalog entries and return the best match while making the output lifetime relationship explicit.

### Required behavior

- Implement a function that accepts a slice of borrowed titles and a query.
- Return the longest title containing the query as `Option<&str>`.
- Return the original title, not a newly allocated lowercase copy.
- Explain which input the output lifetime is tied to.

### Example

```text
titles: ["Rust", "Programming Rust", "Rust in Action"]
query: "Rust"
result: Some("Programming Rust")
```

### Verification checklist

- Verify the longest matching title is returned.
- Verify no match returns `None`.
- Verify a tie follows a documented first-or-last policy.
- Verify the returned slice points to original title text.


- Add a compile-fail example where the result outlives the catalog.
- Verify the query can be dropped after the call while the result remains usable when the catalog lives.

### Done when

The lifetime annotation expresses the catalog relationship without unnecessarily tying the result to the query.
