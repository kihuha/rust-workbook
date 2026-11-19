# Hard 01 — Async chat server

## Project challenge

Build a multi-room chat server and terminal client over TCP. The challenge is to manage many long-lived connections, shared room state, backpressure, and disconnects without blocking the async runtime.

## Required behavior

- Define a newline-delimited protocol for login, room join, message, leave, and error events.
- Require unique active usernames.
- Broadcast room messages to all other members in that room.
- Apply bounded per-client output queues so one slow client cannot consume unlimited memory.
- Remove disconnected clients and notify their rooms.
- Shut down gracefully after stopping new connections.

## Example exchange

```text
C -> {"type":"login","name":"amina"}
C -> {"type":"join","room":"rust"}
C -> {"type":"message","room":"rust","text":"hello"}
S -> {"type":"message","room":"rust","from":"amina","text":"hello"}
```

## Architecture expectations

Separate protocol encoding, connection handling, room coordination, and client UI. Avoid holding locks across `.await`. Use integration tests with real loopback sockets.

## Rust skills combined

Async networking, tasks, channels, shared state, serialization, protocol design, bounded queues, graceful shutdown, and integration testing.

## Done when

Several clients can join different rooms concurrently, malformed frames close or reject only their sender, slow clients follow a documented policy, and shutdown leaves no tasks hanging.

## Brown Rust Book prerequisites

Study [shared ownership](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), [channels and shared state](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [async concurrency](https://rust-book.cs.brown.edu/ch17-00-async-await.html), and [trait objects](https://rust-book.cs.brown.edu/ch18-02-trait-objects.html).

## Suggested tests

### Unit tests

- `T01` — Encode and decode every protocol message without loss.
- `T02` — Duplicate active usernames are rejected.
- `T03` — Joining and leaving updates room membership exactly once.
- `T04` — A full client queue follows the documented backpressure policy.

### Integration tests

- `T05` — Three loopback clients exchange messages only within their rooms.
- `T06` — A malformed frame disconnects or rejects only its sender.
- `T07` — Graceful shutdown terminates all connection tasks.

## Stretch goals

Add authentication, message history, private messages, or WebSocket transport.
