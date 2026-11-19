# 01 — Backend development: SwiftCart fulfillment API

## Company and service

SwiftCart is a fictional regional e-commerce marketplace. Its checkout system needs an order service that accepts purchases, reserves stock, records payment state, and exposes fulfillment updates to merchants.

Build a production-minded REST API named `swiftcart-orders`. The service must support multiple merchants without allowing one merchant to read or mutate another merchant’s data. This is the deliberate bare-bones project: implement the networking and HTTP boundary yourself so you understand the work later frameworks perform.

## Brown Rust Book prerequisites

Study [modules and public APIs](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [tests](https://rust-book.cs.brown.edu/ch11-00-testing.html), [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), and the [standard-library web server project](https://rust-book.cs.brown.edu/ch21-00-final-project-a-web-server.html).

## Users and actors

- Checkout clients create orders.
- Merchant dashboards inspect and advance fulfillment.
- Payment services post signed payment events.
- Operations staff investigate requests through logs, metrics, and traces.

## Required API

All `/v1` routes require a bearer token containing `merchant_id` and scopes. JSON errors use `{ "code", "message", "request_id", "details" }`.

| Method | Endpoint | Purpose |
| --- | --- | --- |
| `POST` | `/v1/orders` | Create an order using an `Idempotency-Key` header |
| `GET` | `/v1/orders/{order_id}` | Read one merchant-owned order |
| `GET` | `/v1/orders` | List orders using cursor pagination and status filters |
| `POST` | `/v1/orders/{order_id}/cancel` | Cancel an eligible order |
| `POST` | `/v1/orders/{order_id}/fulfillment` | Advance `pending -> packed -> shipped -> delivered` |
| `POST` | `/v1/payment-events` | Accept a signed provider event and update payment state |
| `GET` | `/health/live` | Process liveness without dependency checks |
| `GET` | `/health/ready` | Database and migration readiness |
| `GET` | `/metrics` | Prometheus-compatible metrics |

## Core data model

- `Order`: ID, merchant ID, customer reference, currency, status, totals, version, timestamps.
- `OrderItem`: SKU, quantity, unit price, tax, reserved quantity.
- `Payment`: provider reference, amount, status, event timestamp.
- `IdempotencyRecord`: merchant, key, request hash, response status, response body, expiry.
- `OutboxEvent`: event type, aggregate ID, payload, attempts, published timestamp.

Store money in integer minor units. Use optimistic version checks for state changes. Enforce unique provider event IDs and unique idempotency keys per merchant.

## Business rules

- An order needs at least one item and one currency.
- The server recalculates totals; it must not trust client totals.
- Reusing an idempotency key with the same request returns the original response.
- Reusing the key with different data returns `409 Conflict`.
- Paid or shipped orders cannot be cancelled.
- Fulfillment transitions must be ordered and repeat requests must be idempotent.
- Payment events must pass signature and timestamp-window validation.

## Example request and response

```http
POST /v1/orders HTTP/1.1
Authorization: Bearer <merchant-token>
Idempotency-Key: checkout-8421
Content-Type: application/json

{"currency":"KES","customer_ref":"C-91","items":[{"sku":"BK-01","quantity":2,"unit_minor":4500}]}
```

```json
{
  "id": "ord_01JAZ",
  "status": "pending_payment",
  "currency": "KES",
  "subtotal_minor": 9000,
  "version": 1
}
```

## Fixed learning constraints and open decisions

The standard-library HTTP server and blocking PostgreSQL driver below are fixed learning constraints:
they make framing, bounded threads, and blocking I/O visible before frameworks hide them. The
learner must still decide transaction boundaries, reservation concurrency, idempotency storage,
outbox scheduling, indexes, overload policy, and migration/rollback. Record at least two viable
alternatives for each hard-to-reverse choice.

## Architecture expectations

### Required bare-bones stack

- Use `std::net::TcpListener` and `TcpStream`; do not use Tokio or another async runtime.
- Use a bounded standard-library thread pool and bounded work queue; do not create one unbounded thread per connection.
- Parse the required subset of HTTP/1.1 yourself. Do not use Axum, Hyper, Actix Web, Rocket, Tower, or another HTTP/server framework.
- Implement request-line/header parsing, `Content-Length` framing, body limits, timeouts, keep-alive policy, response serialization, and connection closure deliberately.
- You may use focused libraries for JSON, identifiers, cryptography, and the synchronous `postgres` driver. Do not use SQLx, SeaORM, Diesel, or another database framework in this project; persistence calls must remain visibly blocking and stay off connection-handling critical sections.
- Terminate TLS at a reverse proxy for this exercise. Do not invent cryptography or deploy the educational parser directly to the public internet.

Keep transport parsing separate from routing, application services, domain rules, and repository traits. Put external publication behind an outbox worker. Apply database migrations in a controlled deployment step. Document exactly which HTTP/1.1 features are supported and return bounded errors for unsupported transfer encodings, malformed headers, slow bodies, and oversized requests.

The goal is not to recreate a general-purpose web framework. The goal is to experience protocol framing, bounded concurrency, backpressure, connection cleanup, and graceful shutdown once before relying on the [framework prerequisite track](../00-prerequisite/README.md) for later projects.

## Security and privacy

- Validate tenant ownership in the service and query layers.
- Use parameterized queries and bounded request bodies.
- Never log bearer tokens, payment signatures, or customer identifiers.
- Reject stale webhook timestamps and compare signatures in constant time.
- Define retention and deletion rules for customer references.

## Observability and reliability

- Emit structured logs with request, merchant, and order correlation IDs.
- Measure request count, latency, error rate, database pool saturation, and outbox backlog.
- Trace HTTP, database, and event-publication spans.
- Set timeouts for every external call and use bounded retry with jitter.
- Provide graceful shutdown that stops new traffic and drains in-flight requests.

## Required tests

### Unit tests

- `T01` — Creating an order with two `4500` items produces subtotal `9000`.
- `T02` — Empty items, zero quantity, or mixed currencies are rejected.
- `T03` — Invalid fulfillment transitions preserve the original order version.
- `T04` — Paid and shipped orders cannot be cancelled.

### Integration tests

- `T05` — Repository operations round-trip an order and its items in a real test database.
- `T06` — The same idempotency key and body return the original status and body.
- `T07` — The same key with a different body returns `409`.
- `T08` — A merchant token cannot access another merchant’s order.
- `T09` — Duplicate payment event IDs change order state only once.

### End-to-end and operational tests

- `T10` — Create, pay, pack, ship, and deliver an order through HTTP.
- `T11` — Kill the process after committing an outbox event and verify publication resumes after restart.
- `T12` — Run a load test at the agreed target and verify p95 latency and error-budget limits.
- `T13` — Send malformed, oversized, and unauthorized requests and verify bounded safe failures.
- `T14` — Split request headers and bodies across arbitrary TCP reads and still parse valid requests correctly.
- `T15` — Slow or incomplete clients time out without exhausting the bounded worker pool.
- `T16` — Graceful shutdown stops accepting connections, drains in-flight work to a deadline, and closes remaining sockets.

## Local deployment and production design exercise

Package the service as a non-root OCI image with a read-only filesystem and run the complete exercise
locally with Compose or equivalent. Demonstrate readiness, resource limits, migration, backup/
restore, and rollback on that environment.

Separately propose how TLS, horizontal scaling, secret injection, rolling deployment, and failure
domains would work in production. Kubernetes or a cloud deployment is not required and does not
count as evidence unless its behavior is actually measured.

## Local exercise targets

- Successful create-order p95 latency: below `250 ms` under the declared local workload and test environment.
- No acknowledged order or payment event may be lost.
- In the scripted backup/restore exercise, demonstrate a recovery point under `5 minutes` and complete the documented restore within `30 minutes`; report the actual result rather than claiming a production RPO/RTO.

## Acceptance criteria

The service is complete when a reviewer can deploy it from documentation, execute the end-to-end order lifecycle, observe it, rotate its secrets, recover persisted data, and roll back a faulty release.

## System-design progression

Treat the required scope above as the Senior milestone. Declare a workload with normal and flash-sale
traffic, skewed SKUs, payload sizes, and database limits; then report throughput, p50/p95/p99,
contention, pool saturation, and rejection behavior.

For Staff practice, add expiring reservations, an injected clock, high-contention inventory,
reconciliation for stuck order/payment/reservation states, and a choice between a local transaction,
outbox, or saga across split services. Migrate one schema and event version while old workers run.

For Principal practice, define regional inventory ownership, what may continue during region
isolation, tenant/merchant tiers, RPO/RTO, capacity and cost assumptions, team boundaries, and a
phased migration with shadow verification and rollback. Use the
[PostgreSQL isolation documentation](https://www.postgresql.org/docs/current/transaction-iso.html),
[SRE books](https://sre.google/books/), and the
[curriculum rubric](../../../research/system-design-curriculum/evaluation-rubric.md) during review.

## Optional extensions

Add stock reservation expiry, partial fulfillment, refunds, event-bus consumers, or OpenAPI client generation.
