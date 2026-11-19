# 09 — Production integration capstone

## Goal

Build `dispatch-hub`, a small multi-tenant delivery-request service that combines the entire prerequisite stack. It accepts idempotent dispatch requests, calls the unreliable parcel connector asynchronously, exposes status, and sends signed result webhooks.

This is the readiness gate for PayForge and BankBridge. Reuse concepts and small internal libraries from prior challenges, but do not copy their applications wholesale.

## Required stack

- Tokio runtime with owned tasks, bounded channels, cancellation, and graceful shutdown.
- Axum/Serde API with typed extractors, validation, stable errors, and thin handlers.
- Tower middleware for identity, request IDs, tracing, body/timeout/concurrency limits, load shedding, and keyed rate limiting.
- PostgreSQL with SQLx migrations, explicit transactions, constraints, locking, idempotency, and outbox.
- SeaORM for an operator-facing tenant/user/read-model slice; document SQLx escape hatches.
- Browser operator session plus opaque tenant API keys and scoped authorization.
- Reqwest connector and webhook client tested through Wiremock failures.
- `tracing`, OpenTelemetry, Prometheus-compatible metrics, validated configuration, and redacted secrets.

## Required API and workflows

- Create tenants, operator users, API keys, and webhook endpoints through appropriately separated admin flows.
- `POST /v1/dispatches` accepts an idempotency key and persists the dispatch plus outbox job atomically.
- A worker claims jobs with a lease, calls the carrier with a stable operation ID, and reconciles ambiguity.
- `GET /v1/dispatches/{id}` and cursor listing enforce tenant ownership in SQL.
- Completion creates one immutable event and a signed webhook delivery job.
- Webhook retries use bounded exponential backoff, persist attempts, and protect against private-network destinations.
- Revoking a tenant API key blocks new requests immediately without corrupting already accepted work.

## Failure scenarios

Prove behavior for duplicate client requests, database unavailability, pool exhaustion, worker crash at every checkpoint, carrier timeout after success, carrier rate limiting, malformed response, webhook outage, telemetry-exporter outage, and shutdown during active work.

## Required tests

- `T01` — Domain rules pass without Axum, Tokio, or a database.
- `T02` — Repository tests run against disposable PostgreSQL and verify tenant scoping and transactions.
- `T03` — In-process Axum tests cover every route, error class, and middleware policy.
- `T04` — Concurrent identical idempotent creates produce one dispatch and one carrier operation.
- `T05` — Worker crash/restart never loses accepted work or repeats a confirmed external side effect.
- `T06` — Key rotation/revocation, sessions, scopes, CSRF, and cross-tenant attacks follow policy.
- `T07` — Rate, concurrency, timeout, and load-shed controls meet bounded-overload expectations.
- `T08` — Signed webhooks retry, deduplicate, rotate secrets, and reject unsafe destinations.
- `T09` — Logs/traces/metrics correlate a request without leaking canary secrets or high-cardinality IDs.
- `T10` — A real-socket end-to-end test exercises create through final webhook delivery.
- `T11` — A load test meets declared p95/error targets without unbounded queues or pool waits.
- `T12` — Backup/restore, migrations, secret rotation, shutdown, and rollback drills follow written runbooks.

## Deliverables

Provide a Cargo workspace, locked dependencies, development Compose environment, OCI image, migration commands, OpenAPI document, architecture and threat-model diagrams, data dictionary, dashboards, alerts, runbooks, and an operations-focused README.

## Exit criteria

You may start PayForge or BankBridge when a reviewer can deploy Dispatch Hub from scratch, trace and recover a failed operation, demonstrate tenant isolation and overload behavior, rotate credentials, restore data, and roll back a release using only your documentation.
