# Hard 10 — Event-sourced order system

## Project challenge

Build an order service whose state is reconstructed from immutable domain events. The challenge is to enforce command rules, append events atomically, and evolve stored data safely.

## Required behavior

- Handle commands to create an order, add or remove items, submit, pay, cancel, and ship.
- Emit explicit events such as `OrderCreated`, `ItemAdded`, and `OrderPaid`.
- Rebuild aggregate state by replaying an order's event stream.
- Reject invalid transitions, such as shipping an unpaid order.
- Use optimistic version checks to reject concurrent stale writes.
- Build a separate read model for order summaries.

## Example event stream

```text
v1 OrderCreated { order_id: "O-7" }
v2 ItemAdded { sku: "BK-1", quantity: 2, unit_cents: 4500 }
v3 OrderSubmitted
v4 OrderPaid { cents: 9000 }
v5 OrderShipped
```

## Architecture expectations

Separate commands, events, aggregate state, event store, application service, and projections. Events are facts and must not be edited after append.

## Rust skills combined

Domain modeling, enums, event replay, optimistic concurrency, append-only storage, projections, serialization, versioning, traits, and invariant tests.

## Done when

Replay produces the same state as live command handling, stale versions fail without appending, invalid commands emit no events, and projection rebuilds are deterministic.

## System-design foundation

Treat events as a compatibility contract, make optimistic conflicts observable, rebuild projections,
and migrate one event version without rewriting history. Continue with SwiftCart, PayForge, and
[EventLog](../../real-world-usage/11-messaging-streaming/01-eventlog/README.md).

### Foundation evidence

- `F01` — Generate concurrent command histories and assert optimistic conflicts append nothing while replay equals live state.
- `F02` — Add one event version/upcaster, rebuild old/new projections, compare results, and demonstrate rollback without editing history.

## Brown Rust Book prerequisites

Study [structs](https://rust-book.cs.brown.edu/ch05-00-structs.html), [event enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [trait boundaries](https://rust-book.cs.brown.edu/ch10-02-traits.html), [design tradeoffs](https://rust-book.cs.brown.edu/ch18-05-design-challenge.html), and [patterns](https://rust-book.cs.brown.edu/ch19-00-patterns.html).

## Suggested tests

### Unit tests

- `T01` — Each valid command emits the expected events.
- `T02` — Invalid transitions emit no events.
- `T03` — Replaying the example stream produces a shipped, paid order.
- `T04` — Applying live events and replaying stored events produce equal state.

### Integration tests

- `T05` — A stale expected version appends nothing.
- `T06` — Rebuilding the read model twice produces identical summaries.
- `T07` — Restart and replay preserve order state and stream version.

## Stretch goals

Add snapshots, event version migration, idempotency keys, or an async projection worker.
