---
schema: 1
kind: chapter
chapter: 19
slug: patterns-and-matching
title: "Patterns and Matching"
---

# Chapter 19 — Patterns and Matching

## Book alignment

Study [Patterns and Matching](https://rust-book.cs.brown.edu/ch19-00-patterns.html): pattern locations, refutable and irrefutable patterns, destructuring, guards, `@`, and ignored values.

## Mastery goals

- Choose a valid pattern for each syntactic position.
- Destructure nested structs, enums, tuples, and references.
- Use guards, ranges, bindings, and ignored fields precisely.
- Explain refutability rather than relying on compiler trial and error.

<!-- challenge
id: "19.1"
title: "Event pattern router"
status: "not-started"
-->

## Challenge 19.1 — Event pattern router

### Brief

Route nested application events using precise patterns without manually unpacking every field.

### Required behavior

- Model user, system, and network events with nested data.
- Use destructuring to extract only relevant fields.
- Use a match guard for high-priority events.
- Use `@` to bind a value while checking a range.
- Avoid catch-all arms that hide meaningful variants.

### Expected function and `main.rs` flow

```rust
fn route(event: &Event) -> Route;
```

`main` constructs one example of each event variant, borrows each event into `route`, then prints the event and route. `route` contains the destructuring, guard, and `@` patterns; it does not print or consume owned event fields.

### Example

```text
Login { user: "amina", attempts: 1 } -> normal authentication
Login { user: "kamau", attempts: 5 } -> security review
Packet { port: 443, bytes: 1200 } -> secure traffic on port 443
```

### Suggested tests

#### Unit tests

- `T01` — Verify login attempts `1` route normally.
- `T02` — Verify attempts `5` trigger the guard.
- `T03` — Verify `port @ 1..=1023` retains the matched port.
- `T04` — Verify ignored metadata does not need to be cloned or formatted.

#### Integration tests

- `T05` — Verify every event variant produces a route.
- `T06` — Verify adding a new meaningful variant creates an exhaustiveness failure until handled.

### Done when

Patterns communicate data shape and routing intent more clearly than nested field-by-field conditions.

<!-- /challenge -->

<!-- challenge
id: "19.2"
title: "Pattern-driven command interpreter"
status: "not-started"
-->

## Challenge 19.2 — Pattern-driven command interpreter

### Brief

Parse tokenized commands into tuples and use several pattern locations to execute them.

### Required behavior

- Use `while let` to consume commands from a queue.
- Use function-parameter destructuring for coordinates.
- Use `let...else` for required command arguments.
- Distinguish irrefutable `let` patterns from refutable conditional patterns.

### Expected functions and `main.rs` flow

```rust
fn move_by((x, y): &mut (i32, i32), (dx, dy): (i32, i32));
fn execute(commands: VecDeque<Vec<String>>) -> Result<(i32, i32), CommandError>;
```

`main` tokenizes input lines into the queue, calls `execute`, and prints the final position. `execute` owns the `while let` loop and command matching. `move_by` demonstrates parameter destructuring and performs only coordinate mutation.

### Example

```text
MOVE 3 4  -> position (3, 4)
MOVE -1 2 -> position (2, 6)
SHOW      -> position (2, 6)
```

### Suggested tests

#### Unit tests

- `T01` — Verify coordinate tuple destructuring updates both axes.
- `T02` — Verify a missing `MOVE` argument returns an error through `let...else`.
- `T03` — Verify an unknown command is rejected.
- `T04` — Verify an empty queue ends the `while let` loop.

#### Integration tests

- `T05` — Verify the documented command sequence reaches `(2, 6)`.
- `T06` — Add a compile-fail example using a refutable pattern in plain `let` without `else`.

### Done when

Every pattern form is used where its refutability rules and control-flow behavior are appropriate.

<!-- /challenge -->
