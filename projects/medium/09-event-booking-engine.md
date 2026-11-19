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

## Stretch goals

Add reservation expiry, event overlap checks, or an append-only audit log.
