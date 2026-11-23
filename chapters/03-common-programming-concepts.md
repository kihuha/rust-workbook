---
schema: 1
kind: chapter
chapter: 3
slug: common-programming-concepts
title: "Common Programming Concepts"
---

# Chapter 3 — Common Programming Concepts

## Book alignment

Study [Common Programming Concepts](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html): variables, data types, functions, comments, and control flow.

## Mastery goals

- Choose appropriate scalar and compound types.
- Use shadowing and mutability deliberately.
- Write functions with parameters, expressions, and return values.
- Control execution with conditions and loops.

<!-- challenge
id: "03.1"
title: "Tiered delivery calculator"
status: "not-started"
-->

## Challenge 03.1 — Tiered delivery calculator

### Brief

Write functions that validate a parcel and calculate a delivery fee from weight and service level.

### Rules

- Valid weight is `1..=20_000` grams.
- Standard delivery has a `500` cent base fee plus `2` cents for each started `100`-gram group.
- Express delivery has a `1_200` cent base fee plus `5` cents for each started `100`-gram group.
- Therefore, the formulas are `500 + (2 * groups)` for standard and `1_200 + (5 * groups)` for express.
- `groups` is the weight divided by `100`, rounded up. For example, `850` grams uses `9` groups.
- An order value of at least `10_000` cents removes `300` cents from the delivery fee. The fee cannot become negative.

### Expected functions

```rust
fn started_100g_groups(weight_grams: u64) -> u64;
fn validate_weight(weight_grams: u64) -> Result<(), String>;
fn delivery_fee_before_discount(weight_grams: u64, service: &str) -> Result<u64, String>;
fn apply_order_discount(delivery_fee: u64, order_value: u64) -> u64;
fn calculate_delivery_fee(
    weight_grams: u64,
    service: &str,
    order_value: u64,
) -> Result<u64, String>;
```

`apply_order_discount` receives both the calculated delivery fee and the order value. The order value decides eligibility; the `300` cents is subtracted from the delivery fee.

### `main.rs` flow

`main` reads `<weight-grams> <standard|express> <order-value-cents>`, parses the two numbers, and calls only `calculate_delivery_fee`. On success, it prints the documented fields. On failure, it prints the error to standard error and exits unsuccessfully.

```rust
let fee = calculate_delivery_fee(weight_grams, service, order_value)?;
println!("fee: {fee} cents");
```

### Example

```text
weight: 850 g
service: standard
order value: 12000 cents
fee: 218 cents
```

The fee is derived as follows:

```text
groups = ceil(850 / 100) = 9
fee before discount = 500 + (2 * 9) = 518 cents
order value 12000 >= 10000, so discount = 300 cents
final delivery fee = 518 - 300 = 218 cents
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

<!-- /challenge -->

<!-- challenge
id: "03.2"
title: "Number sequence inspector"
status: "not-started"
-->

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

### Expected functions

```rust
fn factors(number: u64) -> Vec<u64>;
fn is_prime(number: u64) -> bool;
fn parity(number: u64) -> &'static str;
```

### `main.rs` flow

`main` reads one positive integer argument, rejects missing, malformed, or zero input, then passes that value to all three functions and prints their results. The calculation functions do not read arguments or print output themselves.

### Verification checklist

- Verify factors of `1` are `[1]` and it is not prime.
- Verify `2` is even and prime.
- Verify factors of `12` are `[1, 2, 3, 4, 6, 12]`.
- Verify `13` is prime.


- Verify input `12` prints all documented fields.
- Verify input `0` returns an error rather than looping indefinitely.

### Done when

The implementation demonstrates variables, types, functions, conditions, and at least one loop form from the chapter.

<!-- /challenge -->
