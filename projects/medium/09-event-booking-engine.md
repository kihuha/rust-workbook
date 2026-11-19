# Medium 09 — Event booking engine

## Project challenge

Build a booking engine with limited capacity and cancellation support. The challenge is to keep reservations, available capacity, and wait-list promotion consistent.

## Required behavior

- Create events with a stable ID, name, start time, and capacity.
- Book a number of seats under a unique reservation ID.
- Reject zero-seat and over-capacity direct bookings.
- Place overflow requests on a FIFO wait-list.
- Cancel reservations and promote eligible wait-list entries.

## Example flow

```text
event capacity: 3
book A for 2 -> confirmed
book B for 2 -> wait-listed
book C for 1 -> confirmed
cancel A     -> B promoted; 1 seat remains
```

Document whether a wait-list request must be filled completely or may be partially filled.

## Rust skills combined

Structs, enums, queues and maps, mutable state transitions, date-time parsing, custom errors, persistence, and scenario tests.

## Done when

Confirmed seats never exceed capacity, IDs remain stable, cancellations are idempotent or return a clear error, and reload preserves wait-list order.

## System-design foundation

State the capacity invariant, add idempotent booking IDs, model expiring holds with an injected
clock, and run concurrent/generated command sequences. Continue with
[SwiftCart](../../real-world-usage/01-backend-development/01-swiftcart/README.md), where contention,
persistence, retries, and reconciliation are production requirements.

### Foundation evidence

- `F01` — Run concurrent/generated reserve, expire, confirm, cancel, and promote commands; capacity never goes negative or over limit.
- `F02` — Repeat operation IDs and advance an injected clock across expiry; prove idempotency and deterministic waitlist promotion.

## Brown Rust Book prerequisites

Study [methods](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html), [state enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [recoverable errors](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), and [automated tests](https://rust-book.cs.brown.edu/ch11-00-testing.html).

## Suggested tests

### Unit tests

- `T01` — Confirmed seats never exceed event capacity.
- `T02` — Overflow requests enter the FIFO wait-list.
- `T03` — Cancelling A promotes B according to the documented fill policy.
- `T04` — Zero-seat requests are rejected.

### Integration tests

- `T05` — The example flow ends with B confirmed and one seat available.
- `T06` — Save and reload without changing reservation IDs or wait-list order.

## Stretch goals

Add reservation expiry, event overlap checks, or an append-only audit log.
