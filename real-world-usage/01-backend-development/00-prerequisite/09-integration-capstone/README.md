# 09 — Production integration capstone

## Goal

Build `dispatch-hub`, a small multi-tenant delivery-request service that combines the entire prerequisite stack. It accepts idempotent dispatch requests, calls the unreliable parcel connector asynchronously, exposes status, and sends signed result webhooks.

This is the readiness gate for PayForge and BankBridge. Reuse concepts and small internal libraries from prior challenges, but do not copy their applications wholesale.

## Before you start

Begin only after the graduation tests in challenges 01–04 pass and you have working examples from 06–08. Review challenge 05's comparison, but choose SQLx for the write path here. This capstone is integration practice: if a milestone introduces its core concept for the first time, return to the focused challenge instead of debugging the whole stack.

Create a new Cargo workspace and a fresh database. Reuse small, understood utilities such as an error envelope or cancellation helper; do not copy `taskboard-api` and rename its nouns. Commit after every checkpoint.

## Product brief

Dispatch Hub serves small merchants that send parcels through a carrier. A tenant integration asks Dispatch Hub to create a delivery. Dispatch Hub accepts durable responsibility, contacts the fictional carrier asynchronously, and eventually sends the tenant a signed webhook containing the result. An operator can inspect tenants and failed work in a browser session.

### Actors

- **Tenant integration:** uses an opaque API key to create/read its own dispatches.
- **Operator:** uses a browser session to administer only assigned tenants and inspect failures.
- **Carrier:** the unreliable provider simulated by Wiremock.
- **Tenant webhook receiver:** another Wiremock server that sometimes fails or responds slowly.
- **Worker:** claims durable jobs and performs carrier/webhook attempts.

### Deliberate scope

A dispatch has one parcel, one pickup address, one delivery address, and no pricing, maps, route optimization, cancellation, or real carrier account. Addresses are fictional test data. The operator UI may be minimal server-rendered HTML or JSON plus a documented session flow; frontend design is not assessed. Run one API process and one worker process locally before considering replicas.

### Example lifecycle

```text
accepted -> submitting -> confirmed
                       \-> rejected
             \-> reconciling -> confirmed | rejected

confirmed dispatch -> webhook pending -> delivered | retry_scheduled | exhausted
```

`POST /v1/dispatches` should return `202 Accepted`, because carrier work is not complete. A minimal request is:

```json
{
  "external_reference": "order-1042",
  "parcel": { "weight_grams": 750 },
  "pickup": { "contact": "Warehouse", "city": "Nairobi" },
  "delivery": { "contact": "Customer", "city": "Nakuru" }
}
```

The response contains a Dispatch Hub ID, status, version, and status URL. Reusing the same idempotency key and canonical request returns that original response; changing the request returns `409`.

## Suggested architecture

```text
crates/
  domain/          # states, commands, policies; no Axum/SQLx
  application/     # use cases and ports
  persistence/     # SQLx repositories and migrations
  carrier/         # provider adapter
  http-api/        # Axum routes, middleware, DTOs
  worker/          # leasing, reconciliation, webhooks
  telemetry/       # shared initialization/field policy
apps/
  server/
  worker/
```

A smaller module layout in one crate is also valid. The important dependency direction is transport/adapters toward application/domain, not the number of crates.

Start with these persisted concepts: `tenants`, `operators`, `sessions`, `api_keys`, `webhook_endpoints`, `dispatches`, immutable `dispatch_events`, `idempotency_records`, `outbox_jobs`, and `webhook_attempts`. Draw the state machine and transaction boundaries before creating tables.

## Build it as vertical slices

### 0. Walking skeleton

Create the workspace, configuration, local Compose PostgreSQL, migrations command, `/health/live`, empty worker loop, and shutdown signal. Add CI commands for format, lint, and tests. Do not add Kubernetes or OpenTelemetry yet.

**Checkpoint:** a new contributor can clone, start dependencies, migrate, run both processes, call liveness, and stop them cleanly using only the README.

### 1. Domain lifecycle without frameworks

Model dispatch states and allowed transitions. Define commands/results and tests for acceptance, provider confirmation/rejection, ambiguity, and terminal behavior. Use deterministic IDs/time through injected interfaces where needed.

**Checkpoint:** domain tests run without Tokio, Axum, SQLx, or environment variables and reject every invalid transition without changing state/version.

### 2. One durable authenticated create/read slice

Add tenants and opaque API keys. Implement `POST /v1/dispatches` and `GET /v1/dispatches/{id}` through Axum, application service, and SQLx. Atomically store dispatch, idempotency result, event, and outbox job. Apply request/body/time/concurrency limits and tenant-scoped queries.

**Checkpoint:** a real PostgreSQL + in-process HTTP test creates and reads a dispatch; duplicate concurrent requests create one dispatch/job; another tenant receives not found.

### 3. Carrier worker and reconciliation

Claim outbox work with a lease, stable operation ID, attempt count, and expiry. Call the Wiremock carrier. Persist each state transition and next action. For ambiguous create outcomes, reconcile by operation lookup before any replay.

**Checkpoint:** exercise success, safe rejection, rate limit, malformed response, “succeeded but response lost,” and crash after every persistence/call boundary. Restart completes all accepted work with one confirmed carrier effect.

### 4. Query and operator slices

Add cursor listing for tenant clients. Add a server-side operator session with CSRF protection and an operator read model implemented in SeaORM; keep dispatch writes in SQLx. Scope operator assignments explicitly.

**Checkpoint:** API-key scopes, session expiry/logout, CSRF, list cursors, and cross-tenant/operator negative cases pass. Captured SeaORM SQL and query count are documented.

### 5. Signed webhook delivery

When a dispatch becomes terminal, atomically create one immutable event and webhook job. Sign the timestamp and raw body with a versioned tenant secret. Persist attempts, use bounded retry/backoff, and define terminal exhaustion. Restrict destinations and redirects.

**Checkpoint:** the receiver verifies a signature, duplicate delivery is identifiable by event ID, secret rotation has a bounded overlap, unsafe destinations fail before network access, and an outage does not block carrier work.

### 6. Observability and overload behavior

Add request IDs, traces across API/outbox/worker/carrier/webhook, low-cardinality metrics, redacted logs, readiness, dashboards, and alerts. Test rate, concurrency, pool, and worker-queue limits with stated local targets.

**Checkpoint:** force each documented failure and locate it using the dashboard/trace/runbook; canary credentials and personal test fields appear in no captured telemetry.

### 7. Packaging and recovery

Build non-root OCI images, validate startup configuration, write migration/rollback and key-rotation procedures, and practice dump/restore. Add a Compose production-like smoke environment before writing orchestrator manifests.

**Checkpoint:** a second clean directory can follow the runbook to build, deploy, create-to-webhook, back up, restore, rotate a credential, shut down under work, and roll back a deliberately bad release.

Only after checkpoint 7 should you run the complete failure matrix and final load test below.

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

The minimum tenant API is `POST /v1/dispatches`, `GET /v1/dispatches/{id}`, and `GET /v1/dispatches` plus live/ready/metrics endpoints. Operator administration may use `/operator/*`; document its exact routes and authorization matrix. Do not add endpoints merely to resemble a real logistics platform.

## Failure scenarios

Prove behavior for duplicate client requests, database unavailability, pool exhaustion, worker crash at every checkpoint, carrier timeout after success, carrier rate limiting, malformed response, webhook outage, telemetry-exporter outage, and shutdown during active work.

## Required tests

These are final acceptance tests. The slice checkpoints are the development order.

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
