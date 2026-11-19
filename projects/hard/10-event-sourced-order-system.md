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

## Stretch goals

Add snapshots, event version migration, idempotency keys, or an async projection worker.
