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

## Stretch goals

Add authentication, message history, private messages, or WebSocket transport.
