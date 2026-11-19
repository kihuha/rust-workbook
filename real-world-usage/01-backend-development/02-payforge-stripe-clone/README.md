# 02 — Backend development: PayForge payment platform

## Project brief

Build `payforge`, a realistic Stripe-style payment backend for internet businesses. Merchants create payment intents, confirm payments through a simulated processor, issue refunds, inspect balances, and receive signed webhooks. The platform must remain correct when clients retry requests, processor responses arrive late, workers crash, or events are delivered more than once.

This is an educational clone of the core server-side workflow, not a card network or a PCI-certified product. Raw card numbers, CVC values, and magnetic-stripe data must never enter your API. The development environment uses single-use test payment-method tokens issued by a simulator; a real deployment would integrate with a certified tokenization provider and undergo security and compliance review.

## Learning outcomes

By completing this project you will practice:

- designing versioned HTTP APIs with stable errors and cursor pagination;
- modelling long-running payment state machines;
- enforcing idempotency across HTTP requests and asynchronous jobs;
- recording money with an immutable double-entry ledger;
- using database transactions, row locks, and an outbox;
- signing, retrying, and observing webhook delivery;
- protecting tenant data, secrets, and auditable operations.

## Brown Rust Book prerequisites

Study [modules and public APIs](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [tests](https://rust-book.cs.brown.edu/ch11-00-testing.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), and [async programming](https://rust-book.cs.brown.edu/ch17-00-async-await.html).

## Actors and trust boundaries

- **Merchant server** uses a secret API key to create and manage payments.
- **Merchant browser** uses a restricted publishable key only with the token simulator.
- **Processor simulator** accepts tokenized payment methods and emits delayed outcomes.
- **PayForge workers** capture funds, post ledger entries, and deliver webhooks.
- **Platform operator** can investigate and replay events, but cannot reveal credentials.

Every merchant-owned row carries `merchant_id`. Authorization must be enforced in repository queries, not only in HTTP handlers.

## Required API

All timestamps are UTC RFC 3339. Money is an ISO 4217 currency plus integer minor units. Mutating merchant requests require `Idempotency-Key`; responses include `Request-Id`.

| Method | Endpoint | Purpose |
| --- | --- | --- |
| `POST` | `/v1/customers` | Create a merchant-scoped customer |
| `POST` | `/v1/payment_methods` | Exchange a simulator token for a reusable test method |
| `POST` | `/v1/payment_intents` | Create an intent with amount, currency, capture mode, and metadata |
| `GET` | `/v1/payment_intents/{id}` | Retrieve current payment state |
| `GET` | `/v1/payment_intents` | List by customer, status, or creation time using a cursor |
| `POST` | `/v1/payment_intents/{id}/confirm` | Start authorization with a payment method |
| `POST` | `/v1/payment_intents/{id}/actions/{action_id}/complete` | Complete a simulator-hosted customer authentication challenge |
| `POST` | `/v1/payment_intents/{id}/capture` | Capture an authorized manual-capture payment |
| `POST` | `/v1/payment_intents/{id}/cancel` | Cancel an eligible intent |
| `POST` | `/v1/refunds` | Create a full or partial refund |
| `GET` | `/v1/balance` | Return pending and available balances by currency |
| `GET` | `/v1/balance_transactions` | List immutable merchant ledger activity |
| `POST` | `/v1/webhook_endpoints` | Register an HTTPS endpoint and subscribed event types |
| `POST` | `/v1/webhook_endpoints/{id}/rotate_secret` | Rotate its signing secret with an overlap window |
| `GET` | `/v1/events` | List the merchant's durable events |
| `POST` | `/v1/events/{id}/redeliver` | Queue an operator-requested webhook redelivery |

Use a consistent error envelope:

```json
{
  "error": {
    "type": "invalid_request",
    "code": "amount_too_small",
    "message": "amount must be at least 50 minor units",
    "param": "amount",
    "request_id": "req_01J..."
  }
}
```

## Core domain model

- `Merchant`: status, default settlement currency, API-key version.
- `Customer`: merchant-scoped identity and non-sensitive metadata.
- `PaymentMethod`: provider token reference, brand, last four, expiry, fingerprint; never raw credentials.
- `PaymentIntent`: amount, currency, capture mode, status, client secret hash, version.
- `Charge`: processor reference, authorized/captured/refunded totals, outcome.
- `Refund`: amount, reason, status, processor reference.
- `LedgerAccount`: merchant/currency/purpose tuple such as processor clearing, pending, available, fee revenue, or refund reserve.
- `LedgerTransaction` and `LedgerEntry`: immutable transaction header and balanced debit/credit lines.
- `Event`: immutable, versioned snapshot of a public event.
- `WebhookEndpoint` and `WebhookDelivery`: subscription, secret version, attempts, next attempt, terminal state.
- `IdempotencyRecord`: merchant, key, endpoint, request hash, response snapshot, expiry.

Use PostgreSQL constraints for positive amounts, supported currency/capture combinations, uniqueness of processor references, and `sum(debits) = sum(credits)` at the ledger transaction boundary.

## Payment lifecycle

Support these intent states:

```text
requires_payment_method -> requires_confirmation -> processing
processing -> requires_action -> processing
processing -> requires_capture -> succeeded
processing -> succeeded
requires_* or requires_capture -> canceled
```

Declines return to `requires_payment_method` with a safe decline code. A payment requiring 3-D Secure or equivalent strong customer authentication enters `requires_action` with a short-lived, single-use action reference safe for the merchant client. Completing, failing, abandoning, or expiring the challenge follows an explicit transition and replay policy; no processor secret is exposed to the browser. A processor timeout remains `processing` until reconciliation proves the outcome; never guess that a timed-out authorization failed. A successful automatic capture atomically records the charge, ledger transaction, intent transition, and outbox event.

Manual capture may capture less than the authorized amount once. Refunds may be partial, but cumulative refunds cannot exceed the captured amount even under concurrent requests.

## Idempotency and concurrency contract

- Scope idempotency keys to merchant, HTTP method, and route family.
- Hash a canonical form of the request. Reuse with different input returns `409 idempotency_key_reused`.
- Store the original status and body in the same transaction as the mutation.
- Concurrent identical requests wait for or return the one committed result.
- Lock the intent while applying transitions and use a uniqueness constraint as the final duplicate defense.
- Workers claim jobs with leases or `FOR UPDATE SKIP LOCKED`; expired leases are recoverable.
- Processor commands carry stable operation IDs so reconciliation and retries cannot double-charge.

## Immutable ledger rules

The ledger is the financial source of truth; mutable balance columns are only rebuildable caches.

- Every ledger transaction has at least two entries and balances to zero per currency.
- Never update or delete a posted entry. Correct mistakes with a linked reversal.
- Authorization does not create an available merchant balance.
- Capture moves value through processor clearing and merchant pending accounts and records the fee separately.
- Refunds reverse merchant value and fees according to a documented policy.
- A settlement worker moves eligible pending funds to available funds after a configurable delay.
- Rebuilding balances from entries must exactly match the served balance snapshot.

Write a short accounting note showing the entries for a `10,000` capture, `300` fee, and `2,000` partial refund.

## Webhook delivery contract

Create events only after the related database transaction commits. Deliver a frozen event payload; later object changes must not alter an old event.

Sign `timestamp.payload` with HMAC-SHA256 and send a header such as `PayForge-Signature: t=...,v1=...`. Retry network errors and `429`/`5xx` responses with exponential backoff and jitter for up to three days. Treat any `2xx` as success. Preserve delivery history, cap response bodies, prevent internal-network destinations, and support secret rotation without invalidating in-flight attempts.

Ordering is best effort. Document that consumers must deduplicate by event ID and retrieve the latest object when ordering matters.

## Processor simulator

Provide deterministic test tokens for success, generic decline, insufficient funds, required customer authentication, failed authentication, expired challenge, delayed success, timeout followed by success, and processor outage. The simulator must support authorization lookup by stable operation ID so a reconciliation worker can resolve ambiguous outcomes.

Introduce configurable latency and duplicate callbacks. Keep this simulator behind a trait so its contract could be implemented by a real processor adapter.

## Security and compliance baseline

- Store API keys and webhook secrets only as hashes or encrypted secret material, and show a secret once at creation.
- Separate publishable and secret key capabilities; support key rotation and immediate revocation.
- Never log tokens, client secrets, authorization headers, or full request bodies.
- Use constant-time signature comparison, bounded JSON, parameterized queries, TLS, and explicit timeouts.
- Rate-limit by credential and merchant, with stricter limits for confirmation and redelivery.
- Reject arbitrary webhook URLs that resolve to loopback, link-local, private, or metadata-service addresses, including after redirects and DNS changes.
- Record audit events for credential changes, endpoint changes, refunds, and manual redelivery.
- Publish a threat model and state clearly what remains outside PCI DSS scope in the educational build.

## Suggested architecture

Use a Cargo workspace with crates such as `payforge-domain`, `payforge-api`, `payforge-store`, `payforge-worker`, `payforge-processor`, and `payforge-cli`. A practical stack is Axum, Tokio, Serde, SQLx, PostgreSQL, `tracing`, and OpenTelemetry. Keep domain transitions and ledger construction independent from HTTP and SQL.

Run API and worker processes separately. PostgreSQL is authoritative. An outbox table is sufficient initially; adding Kafka or NATS must not weaken transactional guarantees.

## Required tests

### Unit and property tests

- `T01` — Generated valid ledger transactions balance to zero per currency.
- `T02` — Invalid payment-state transitions change no state.
- `T03` — Partial refunds never exceed captured value under generated sequences.
- `T04` — Canonical request hashing is stable across JSON object key order.
- `T05` — Webhook signatures accept the active/overlap secret within tolerance and reject tampering or stale timestamps.
- `T06` — Authentication actions are single-use and success, failure, expiry, and replay follow the documented state transitions.

### Database and API integration tests

- `T07` — Two concurrent confirms with one idempotency key cause one processor authorization.
- `T08` — Reusing that key with different input returns `409` and preserves the original response.
- `T09` — Two concurrent refunds cannot over-refund a charge.
- `T10` — A merchant credential cannot observe or mutate another merchant's objects or events.
- `T11` — Killing the worker after processor success but before local completion is repaired by reconciliation without a second charge.
- `T12` — Every successful capture/refund writes a balanced ledger transaction and one public event.
- `T13` — A required authentication challenge can succeed through HTTP, while failed and expired challenges never capture funds.

### End-to-end and operational tests

- `T14` — Create, confirm, capture, partially refund, settle, and reconcile a payment through HTTP.
- `T15` — A failing webhook endpoint retries on schedule, survives restart, then succeeds without changing its event payload.
- `T16` — Rotate an endpoint secret while queued deliveries remain verifiable.
- `T17` — Rebuild all merchant balances from the ledger and compare them with API results.
- `T18` — Load test the stated merchant mix while meeting latency and error targets.
- `T19` — Fuzz JSON decoding, signature parsing, and state-machine command sequences.

## Operations and observability

Emit structured logs and traces with request, merchant, intent, event, and delivery IDs, while redacting secrets. Track HTTP rate/errors/duration, processor latency and ambiguity, intent age in `processing`, reconciliation backlog, outbox age, webhook attempts, ledger imbalance checks, database pool saturation, and worker lease expiry.

Provide runbooks for stuck processing payments, processor degradation, webhook backlog, leaked API keys, ledger invariant failure, database restore, and safe worker rollback. Add readiness checks that distinguish API availability from worker health.

## Service objectives

- Payment creation and retrieval availability: `99.95%` monthly in the exercise environment.
- p95 create/retrieve latency: under `250 ms` at the declared load, excluding processor confirmation time.
- No duplicate capture caused by PayForge for a single stable operation ID.
- No loss of acknowledged mutations or committed ledger entries.
- Webhook event created within `5 seconds` p99 of a completed payment transition.
- Reconciliation resolves simulated ambiguous outcomes within `2 minutes` p99.

## Deployment and deliverables

Provide non-root OCI images, versioned migrations, local Compose dependencies, production-style configuration, health endpoints, seed scripts, backup/restore steps, and a zero-downtime rollout plan. Include an OpenAPI document, architecture decision records, threat model, data-retention policy, dashboards, alerts, and operator runbooks.

## Acceptance criteria

The project is complete when a reviewer can provision a merchant, run every simulator outcome, prove retry safety, inspect balanced financial entries, receive and verify webhooks, rebuild balances, recover from a killed worker, rotate secrets, restore a backup, and roll back a release using only the supplied documentation.

## Optional extensions

Add subscriptions and invoices, connected sub-merchants, disputes, multi-currency settlement, a hosted checkout page, event streaming, or a read-only merchant dashboard. Treat each extension as a new domain with its own invariants rather than another flag on `PaymentIntent`.
