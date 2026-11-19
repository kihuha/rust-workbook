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

## Stretch goals

Add TTL expiry, snapshots, batch writes, or an in-process client library.
