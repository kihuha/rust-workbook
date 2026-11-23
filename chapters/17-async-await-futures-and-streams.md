---
schema: 1
kind: chapter
chapter: 17
slug: async-await-futures-and-streams
title: "Async, Await, Futures, and Streams"
---

# Chapter 17 — Async, Await, Futures, and Streams

## Book alignment

Study [Fundamentals of Asynchronous Programming](https://rust-book.cs.brown.edu/ch17-00-async-await.html): futures, concurrent composition, dynamic future collections, streams, async traits, tasks, and threads.

## Mastery goals

- Create and await futures through an async runtime.
- Run independent operations concurrently rather than sequentially.
- Work with collections of futures and streams.
- Choose async tasks or OS threads based on workload.

<!-- challenge
id: "17.1"
title: "Concurrent service checker"
status: "not-started"
-->

## Challenge 17.1 — Concurrent service checker

### Brief

Check several simulated or local HTTP services concurrently and preserve one result per service.

### Required behavior

- Start every check before waiting for all results.
- Apply a per-service timeout.
- Limit concurrent checks with a semaphore.
- Return success, timeout, or request error for each service.
- Use a local mock server in automated tests.

### Expected interface and `main.rs` flow

```rust
async fn check_service(
    service: Service,
    timeout: std::time::Duration,
) -> ServiceResult;

async fn check_all(
    services: Vec<Service>,
    timeout: std::time::Duration,
    concurrency: usize,
) -> Result<Vec<ServiceResult>, ConfigError>;
```

Async `main` builds the service list, calls `check_all`, then prints every result and the healthy count. `check_all` acquires semaphore permits and starts checks concurrently; `main` must not await each service in a sequential loop.

### Example

```text
api: 200 OK in 80 ms
search: timeout after 500 ms
billing: connection error
healthy: 1/3
```

### Suggested tests

#### Unit tests

- `T01` — Verify status classification for successful and failed responses.
- `T02` — Verify summary counts include every outcome.
- `T03` — Verify zero concurrency is rejected as configuration error.

#### Integration tests

- `T04` — Verify delayed mock responses complete near the longest delay, not their sum.
- `T05` — Verify concurrency never exceeds the configured limit.
- `T06` — Verify one timeout does not discard successful results.

### Done when

The tests prove actual overlap and every future resolves to a recorded outcome.

<!-- /challenge -->

<!-- challenge
id: "17.2"
title: "Streamed event window"
status: "not-started"
-->

## Challenge 17.2 — Streamed event window

### Brief

Consume an asynchronous stream of events, group them into bounded windows, and stop cleanly on cancellation.

### Required behavior

- Produce numbered events at simulated intervals.
- Consume at most five events per window.
- Flush a partial window when the stream ends.
- Allow a cancellation signal to stop production and consumption.
- Avoid blocking calls inside async tasks.

### Expected interface and `main.rs` flow

```rust
fn event_stream(count: u64, interval: std::time::Duration) -> impl Stream<Item = u64>;
async fn collect_windows<S>(events: S, window_size: usize) -> Vec<Vec<u64>>
where
    S: Stream<Item = u64>;
```

Async `main` creates the stream, passes it to `collect_windows` with a size of `5`, and prints returned windows plus the total event count. Cancellation must reach both producer and consumer; neither function uses `std::thread::sleep`.

### Example

```text
window 1: [1, 2, 3, 4, 5]
window 2: [6, 7, 8]
stream complete: 8 events
```

### Suggested tests

#### Unit tests

- `T01` — Verify five events form one full window.
- `T02` — Verify eight events form windows of five and three.
- `T03` — Verify an empty stream produces no windows.

#### Integration tests

- `T04` — Verify the consumer handles delayed events without blocking the runtime.
- `T05` — Verify cancellation stops the producer and the task terminates.
- `T06` — Verify dynamic stream values can be processed safely when pinning is required.

### Done when

You can explain the roles of the stream, future polling, task scheduling, and cancellation path.

<!-- /challenge -->
