# Chapter 6 — Enums and Pattern Matching

## Book alignment

Study [Enums and Pattern Matching](https://rust-book.cs.brown.edu/ch06-00-enums.html): defining enums, `match`, `if let`, `let...else`, and [Ownership Inventory #1](https://rust-book.cs.brown.edu/ch06-04-inventory.html).

## Mastery goals

- Model mutually exclusive states with data-bearing variants.
- Handle variants exhaustively with patterns.
- Use `Option<T>` instead of sentinel values.
- Predict when matching borrows or moves owned fields.

## Challenge 06.1 — Payment processor

### Brief

Model several payment methods and produce an authorization result without boolean status flags.

### Required behavior

- Define payment variants for cash, card, and mobile money.
- Define result variants for approved, declined, and invalid details.
- Use exhaustive `match` expressions.
- Carry relevant data, such as a receipt code or decline reason, in variants.

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

## Challenge 06.2 — Ownership Inventory: support ticket routing

### Brief

Process tickets whose enum variants contain owned strings. Compare matching by value, by shared reference, and by mutable reference.

### Required behavior

- Define `Ticket::{Bug(String), Question(String), Closed { reason: String }}`.
- Write one function that borrows a ticket to produce a route.
- Write one function that mutably borrows and normalizes open-ticket text.
- Write one function that consumes a ticket to archive its owned text.

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
