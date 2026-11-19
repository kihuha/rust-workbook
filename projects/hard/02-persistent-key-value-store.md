# Hard 02 — Persistent key-value store

## Project challenge

Build a networked key-value database backed by an append-only log. The challenge is to recover valid state after restarts and interrupted writes while keeping reads and writes consistent.

## Required behavior

- Support `SET`, `GET`, `DELETE`, and `SCAN prefix` over a documented TCP protocol.
- Append mutations to a checksummed log before acknowledging success.
- Rebuild the in-memory index by replaying the log at startup.
- Detect and safely ignore or report an incomplete final record.
- Compact the log without losing writes or replacing valid data prematurely.
- Serve multiple clients with a documented concurrency model.

## Example session

```text
SET user:1 Amina -> OK
GET user:1       -> Amina
DELETE user:1    -> DELETED
GET user:1       -> NOT_FOUND
```

## Architecture expectations

Separate protocol, command execution, storage log, index, compaction, and server lifecycle. Inject filesystem paths and use temporary directories in recovery tests.

## Rust skills combined

Async networking, binary or text protocols, file I/O, checksums, indexes, locking, crash recovery, compaction, and concurrency tests.

## Done when

Acknowledged writes survive restart, corrupt records are identified, compaction produces equivalent state, and concurrent tests never observe a partially applied record.

## Brown Rust Book prerequisites

Study [hash maps](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [shared-state concurrency](https://rust-book.cs.brown.edu/ch16-03-shared-state.html), and [unsafe Rust](https://rust-book.cs.brown.edu/ch20-01-unsafe-rust.html) only if the design truly requires it.

## Suggested tests

### Unit tests

- `T01` — Apply `SET`, `GET`, and `DELETE` to the in-memory index.
- `T02` — Reject a record with an invalid checksum.
- `T03` — Replay a valid log into the expected final state.
- `T04` — Ignore or report an incomplete final record according to policy.

### Integration tests

- `T05` — Acknowledged writes survive process restart.
- `T06` — Compaction preserves every visible key and tombstone outcome.
- `T07` — Concurrent clients never observe a partially applied record.

## Stretch goals

Add TTL expiry, snapshots, batch writes, or an in-process client library.
