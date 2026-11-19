# 05 — Networking: StreamGate reverse proxy

## Company and service

StreamGate is a fictional video-learning platform. Its internal services need a low-latency edge proxy that terminates TLS, routes requests by host and path, balances traffic, and protects unhealthy upstreams.

Build `streamgate`, a production Layer-7 reverse proxy. Use established TLS and HTTP crates; do not implement cryptography or HTTP parsing from scratch.

## Brown Rust Book prerequisites

Study [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [smart pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [async](https://rust-book.cs.brown.edu/ch17-00-async-await.html), and [trait objects](https://rust-book.cs.brown.edu/ch18-02-trait-objects.html).

## Required data plane

- Accept HTTP/1.1 and optionally HTTP/2 over TLS.
- Route by exact host, path prefix, and HTTP method.
- Support round-robin and least-active upstream selection.
- Stream request and response bodies with bounded buffering.
- Apply connect, header, idle, and total-request timeouts.
- Preserve or generate request IDs and sanitize forwarding headers.
- Retry only safe, replayable requests under documented conditions.
- Enforce body-size, header-size, connection, and concurrency limits.

## Configuration example

```toml
[[route]]
host = "api.streamgate.test"
path_prefix = "/v1/videos"
methods = ["GET", "POST"]
cluster = "video-api"

[cluster.video-api]
strategy = "least-active"
upstreams = ["10.0.1.10:8080", "10.0.1.11:8080"]
health_path = "/health/ready"
```

## Management API

Bind the management listener to a separate interface with independent authentication.

| Method | Endpoint | Purpose |
| --- | --- | --- |
| `GET` | `/ready` | Config loaded and required workers running |
| `GET` | `/metrics` | Connections, latency, bytes, upstream health, retries |
| `GET` | `/v1/routes` | Effective redacted route configuration |
| `GET` | `/v1/upstreams` | Health and active-request counts |
| `POST` | `/v1/reload` | Validate and atomically activate new configuration |
| `POST` | `/v1/drain` | Stop new connections and drain existing traffic |

## Example routing behavior

```text
GET api.streamgate.test/v1/videos/42
-> route video-api
-> upstream 10.0.1.11:8080
-> 200 OK, request-id sg-8f21
```

## Reliability rules

- Health checks use failure and recovery thresholds to avoid flapping.
- Passive failures influence routing without permanently ejecting an upstream.
- Configuration reload is atomic; invalid config leaves the old version active.
- Graceful drain stops accepting connections and allows bounded completion.
- Client cancellation must cancel upstream work and release permits.
- Backpressure must prevent slow clients from causing unbounded memory use.

## Security

- Use modern TLS defaults, automated certificate reload, and optional upstream mTLS.
- Normalize paths and reject request smuggling ambiguities.
- Strip untrusted hop-by-hop and forwarded headers.
- Protect management endpoints separately from data-plane traffic.
- Never log authorization headers, cookies, or full sensitive query strings.

## Required tests

### Unit tests

- `T01` — Route precedence selects the most specific matching route.
- `T02` — Load-balancing strategies distribute only across healthy upstreams.
- `T03` — Retry policy rejects non-replayable requests.
- `T04` — Forwarded-header sanitization removes client spoofing.

### Integration tests

- `T05` — Proxy requests and streaming bodies through local upstream servers.
- `T06` — Timeout and client cancellation release connections and permits.
- `T07` — Failed health checks eject an upstream and recovery restores it.
- `T08` — Invalid configuration reload leaves active routes unchanged.
- `T09` — TLS certificate reload serves the new certificate without restart.

### Performance and resilience tests

- `T10` — Meet the agreed requests-per-second and p99 latency targets.
- `T11` — Slow-client and slow-upstream tests remain within memory bounds.
- `T12` — Fuzz route configuration and HTTP boundary handling.
- `T13` — Drain during load and verify no accepted request has an undefined outcome.

## Deployment target

Build a minimal non-root OCI image with a read-only filesystem and writable runtime directory. Deploy at least two replicas behind a network load balancer with disruption budgets and topology spread.

Provide dashboards, alerts, certificate rotation, configuration rollout, canary, rollback, and incident runbooks. Pin and audit all network and TLS dependencies.

## Service-level objectives

- Availability: `99.99%` monthly.
- Added proxy latency: below `10 ms` p99 inside the target region under normal load.
- Configuration reload must not drop established connections.
- No unbounded queues or buffers under adversarial slow traffic.

## Acceptance criteria

The proxy routes, balances, times out, reloads, drains, and recovers under automated fault injection while maintaining documented resource and latency limits.

## Optional extensions

Add WebSocket proxying, circuit breakers, traffic shadowing, weighted canaries, or pluggable request policies through WebAssembly.
