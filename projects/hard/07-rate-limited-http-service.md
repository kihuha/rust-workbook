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

## Stretch goals

Add expiring links, click analytics, authentication, or distributed rate-limit storage.
