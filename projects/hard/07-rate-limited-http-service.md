# Hard 07 — Rate-limited HTTP service

## Project challenge

Build a production-style HTTP API for managing short links. The challenge is to combine validated domain behavior with concurrent request handling, rate limits, observability, and graceful failure.

## Required behavior

- Create, resolve, list, and delete short links.
- Validate target URLs and generate collision-resistant short codes.
- Persist links through a repository interface.
- Apply a token-bucket rate limit per client identity.
- Return structured JSON errors and correct HTTP status codes.
- Emit request IDs, structured logs, latency metrics, and graceful shutdown behavior.

## Example request

```text
POST /links {"url":"https://example.com/docs"}
201 {"code":"aB39x","url":"https://example.com/docs"}

GET /aB39x
302 Location: https://example.com/docs
```

## Architecture expectations

Keep handlers thin. Separate domain service, repository, rate limiter, configuration, and HTTP representation. Test with an in-memory repository and the framework's test client.

## Rust skills combined

Async HTTP, domain services, traits, shared state, rate-limiting algorithms, structured errors, observability, configuration, and integration tests.

## Done when

Concurrent creation is collision-safe, rate-limit headers are correct, storage errors map predictably, logs contain no secrets, and in-flight requests finish during graceful shutdown.

## System-design foundation

Define the client identity and quota contract, bound concurrency and bodies, test saturation, and
report p50/p95/p99 plus rejected load. Continue with
[StreamGate](../../real-world-usage/05-networking/01-streamgate/README.md) and TenantPlane.

### Foundation evidence

- `F01` — Hold handlers slow and send oversized/bursty requests; assert body, queue, concurrency, and per-client quota bounds.
- `F02` — Load from below to beyond saturation; report accepted/rejected rate, latency p50/p95/p99, memory, and graceful drain outcomes.

## Brown Rust Book prerequisites

Study [public APIs](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [async programming](https://rust-book.cs.brown.edu/ch17-00-async-await.html), and [dynamic abstraction](https://rust-book.cs.brown.edu/ch18-02-trait-objects.html).

## Suggested tests

### Unit tests

- `T01` — Reject invalid target URLs.
- `T02` — Token-bucket refill and consumption follow the configured clock.
- `T03` — Repository failures map to structured service errors.
- `T04` — Secret values are absent from formatted logs.

### Integration tests

- `T05` — Create and resolve the example short link through HTTP.
- `T06` — Requests over the limit receive the documented status and headers.
- `T07` — Graceful shutdown lets accepted requests finish and rejects new work.

## Stretch goals

Add expiring links, click analytics, authentication, or distributed rate-limit storage.
