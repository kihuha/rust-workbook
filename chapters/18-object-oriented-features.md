# Chapter 18 — Object-Oriented Programming Features

## Book alignment

Study [Object-Oriented Programming Features](https://rust-book.cs.brown.edu/ch18-00-oop.html): encapsulation, trait objects, state patterns, [Ownership Inventory #4](https://rust-book.cs.brown.edu/ch18-04-inventory.html), and [Design Trade-offs](https://rust-book.cs.brown.edu/ch18-05-design-challenge.html).

## Mastery goals

- Use encapsulation without relying on inheritance.
- Compare enums, generics, and trait objects.
- Apply dynamic dispatch to heterogeneous values.
- Evaluate ownership and API tradeoffs in state-based designs.

## Challenge 18.1 — Heterogeneous notification service

### Brief

Send notifications through email, SMS, and console implementations stored in one collection.

### Required behavior

- Define a `Notifier` trait with a borrowing `send` method.
- Store heterogeneous implementations behind trait objects.
- Keep implementation-specific fields private.
- Return structured delivery outcomes rather than printing inside domain logic.
- Explain why a generic `Vec<T>` alone cannot hold several concrete notifier types.

### Example

```text
message: "Build finished"
email -> delivered to amina@example.com
sms -> delivered to 0712345678
console -> Build finished
```

### Suggested tests

#### Unit tests

- `T01` — Verify each notifier produces its documented delivery outcome.
- `T02` — Verify invalid implementation-specific configuration is rejected.
- `T03` — Verify sending borrows the notifier and message.

#### Integration tests

- `T04` — Verify one `Vec<Box<dyn Notifier>>` dispatches to all implementations.
- `T05` — Verify a new notifier can be added without changing the broadcast loop.

### Done when

Dynamic dispatch solves a real heterogeneous-collection requirement and encapsulation prevents invalid external mutation.

## Challenge 18.2 — Ownership Inventory and design comparison

### Brief

Model a document workflow in two ways: a runtime state enum and state-specific types that consume `self` during transitions. Compare their ownership and API tradeoffs.

### Required behavior

- Support draft, review, published, and rejected states.
- Allow editing only while in draft.
- Require review before publication.
- Make invalid transitions return errors in the enum design.
- Make at least one invalid transition unrepresentable in the type-state design.

### Example

```text
Draft -> request_review -> Review
Review -> approve -> Published
Published content: "Ownership notes"
Published -> edit -> rejected by API
```

### Suggested tests

#### Unit tests

- `T01` — Verify draft content can be edited.
- `T02` — Verify an enum-based draft cannot publish directly.
- `T03` — Verify review can become published or rejected.
- `T04` — Verify published content is readable but not editable.

#### Integration tests

- `T05` — Verify both designs complete the valid workflow with equivalent output.
- `T06` — Add a compile-fail example for an invalid type-state transition.
- `T07` — Document whether states, transitions, or runtime extensibility are easier in each design.

### Done when

The design choice is justified in terms of ownership, invalid states, dispatch, extensibility, and caller ergonomics.
