# Chapter 3 — Common Programming Concepts

## Book alignment

Study [Common Programming Concepts](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html): variables, data types, functions, comments, and control flow.

## Mastery goals

- Choose appropriate scalar and compound types.
- Use shadowing and mutability deliberately.
- Write functions with parameters, expressions, and return values.
- Control execution with conditions and loops.

## Challenge 03.1 — Tiered delivery calculator

### Brief

Write functions that validate a parcel and calculate a delivery fee from weight and service level.

### Rules

- Valid weight is `1..=20_000` grams.
- Standard costs `500 + 2` cents per started `100` grams.
- Express costs `1200 + 5` cents per started `100` grams.
- Orders worth at least `10_000` cents receive a `300` cent discount, never below zero.

### Example

```text
weight: 850 g
service: standard
order value: 12000 cents
fee: 218 cents
```

### Verification checklist

- Verify `850 g` uses nine 100-gram units.
- Verify standard delivery for `850 g` before discount is `518` cents.
- Verify the qualifying discount produces `218` cents.
- Verify weights `0` and `20_001` are rejected.


- Verify valid CLI values print the expected fee.
- Verify an unknown service level exits with a clear error.

### Done when

Named functions express validation, rounding, base pricing, and discount rules without duplicated branches.

## Challenge 03.2 — Number sequence inspector

### Brief

Accept a positive integer and report its factors, parity, and whether it is prime. Use loops and expressions rather than collection helpers introduced later.

### Example

```text
input: 12
parity: even
factors: 1 2 3 4 6 12
prime: false
```

### Required behavior

- Reject zero and negative integers.
- Print factors in ascending order.
- Define `1` as not prime.
- Keep factor and primality logic in functions.

### Verification checklist

- Verify factors of `1` are `[1]` and it is not prime.
- Verify `2` is even and prime.
- Verify factors of `12` are `[1, 2, 3, 4, 6, 12]`.
- Verify `13` is prime.


- Verify input `12` prints all documented fields.
- Verify input `0` returns an error rather than looping indefinitely.

### Done when

The implementation demonstrates variables, types, functions, conditions, and at least one loop form from the chapter.
