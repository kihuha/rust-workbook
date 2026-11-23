---
schema: 1
kind: chapter
chapter: 6
slug: enums-and-pattern-matching
title: "Enums and Pattern Matching"
---

# Chapter 6 — Enums and Pattern Matching

## Book alignment

Study [Enums and Pattern Matching](https://rust-book.cs.brown.edu/ch06-00-enums.html): defining enums, `match`, `if let`, `let...else`, and [Ownership Inventory #1](https://rust-book.cs.brown.edu/ch06-04-inventory.html).

## Mastery goals

- Model mutually exclusive states with data-bearing variants.
- Handle variants exhaustively with patterns.
- Use `Option<T>` instead of sentinel values.
- Predict when matching borrows or moves owned fields.

<!-- challenge
id: "06.1"
title: "Payment processor"
status: "not-started"
-->

## Challenge 06.1 — Payment processor

### Brief

Model several payment methods and produce an authorization result without boolean status flags.

### Required behavior

- Define payment variants for cash, card, and mobile money.
- Define result variants for approved, declined, and invalid details.
- Use exhaustive `match` expressions.
- Carry relevant data, such as a receipt code or decline reason, in variants.

### Expected functions and `main.rs` flow

```rust
fn authorize(payment: PaymentMethod) -> AuthorizationResult;
fn format_result(result: &AuthorizationResult) -> String;
```

`main` constructs each example `PaymentMethod`, passes ownership to `authorize`, then borrows the returned result for `format_result` and prints the message. `authorize` contains the exhaustive payment match; `format_result` contains the exhaustive result match.

### Example

```text
payment: MobileMoney { phone: "0712345678", cents: 2500 }
result: Approved { receipt: "MM-2500" }
message: Approved 2500 cents with receipt MM-2500
```

### Verification checklist

- Verify valid cash is approved without account details.
- Verify an empty mobile number returns invalid details.
- Verify a card decline carries its reason.
- Verify zero cents is rejected for every payment method.


- Verify each payment variant produces a user-facing message.
- Verify adding a new result variant forces exhaustive handlers to be updated.

### Done when

Impossible combinations cannot be constructed through unrelated booleans or optional fields.

<!-- /challenge -->

<!-- challenge
id: "06.2"
title: "Ownership Inventory: support ticket routing"
status: "not-started"
-->

## Challenge 06.2 — Ownership Inventory: support ticket routing

### Brief

Process tickets whose enum variants contain owned strings. Compare matching by value, by shared reference, and by mutable reference.

### Required behavior

- Define `Ticket::{Bug(String), Question(String), Closed { reason: String }}`.
- Write one function that borrows a ticket to produce a route.
- Write one function that mutably borrows and normalizes open-ticket text.
- Write one function that consumes a ticket to archive its owned text.

### Expected functions and `main.rs` flow

```rust
fn route(ticket: &Ticket) -> &'static str;
fn normalize(ticket: &mut Ticket);
fn archive_text(ticket: Ticket) -> String;
```

`main` creates one mutable ticket, calls `route(&ticket)`, calls `normalize(&mut ticket)`, then passes the ticket by value to `archive_text`. The call order demonstrates shared borrowing, mutable borrowing, then ownership transfer.

### Example

```text
ticket: Bug("  crash on save  ")
route: engineering
normalized: Bug("crash on save")
archive text: crash on save
```

### Verification checklist

- Verify routing borrows and leaves the ticket usable.
- Verify normalization mutates text without changing the variant.
- Verify archiving moves the owned text into the archive.
- Verify `if let` handles only the intended variant.


- Add a compile-fail example that moves a string field and then reuses the original ticket.
- Verify the borrow, mutate, and consume sequence succeeds when ordered correctly.

### Done when

You can explain the ownership permissions created by each match style before compiling.

<!-- /challenge -->
