# Medium 04 — Async website monitor

## Project challenge

Build an asynchronous monitor that checks configured web endpoints repeatedly. The challenge is to manage concurrency, timeouts, and partial failure without letting one service block the others.

## Required behavior

- Load service names, URLs, intervals, and timeout limits from a config file.
- Check several endpoints concurrently.
- Record HTTP status, latency, timeout, or request error per service.
- Limit the number of requests in flight.
- Write each completed check to an append-only history file.

## Example report

```text
api      200  84 ms
search   timeout after 1000 ms
billing  connection error
summary  1/3 healthy
```

## Architecture expectations

Use a local mock HTTP server in tests. Put scheduling, HTTP execution, result classification, and history storage behind separate interfaces where useful.

## Rust skills combined

Async/await, futures, semaphores, timeouts, serialization, configuration parsing, error handling, traits, and async tests.

## Done when

Slow checks overlap, concurrency never exceeds the configured limit, one failure does not cancel other checks, and history rows are complete and parseable.

## Stretch goals

Add retry backoff, rolling uptime percentages, or graceful Ctrl-C shutdown.
