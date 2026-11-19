# 03 — Backend development: BankBridge open-banking platform

## Project brief

Build `bankbridge`, a realistic Plaid-style aggregation backend. Applications create short-lived link sessions, users authorize an institution connection, and clients retrieve normalized accounts, balances, and incrementally synchronized transactions. Institution credentials and access tokens are highly sensitive; consent, revocation, degraded providers, and inconsistent upstream data are first-class domain concerns.

The default build connects to a deterministic bank sandbox. It must not collect real online-banking passwords. A production connector would use institution-hosted OAuth or a regulated aggregation partner and would require legal, security, privacy, and regional open-banking review.

## Learning outcomes

By completing this project you will practice:

- isolating unreliable third-party integrations behind connector traits;
- implementing OAuth state, PKCE, consent, and token lifecycles;
- normalizing institution-specific data without losing provenance;
- building cursor-based incremental synchronization;
- coordinating scheduled jobs, rate limits, retries, and webhooks;
- encrypting sensitive data and enforcing tenant/user boundaries;
- designing privacy deletion, audit, and incident workflows.

## Brown Rust Book prerequisites

Study [ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [testing](https://rust-book.cs.brown.edu/ch11-00-testing.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), and [async programming](https://rust-book.cs.brown.edu/ch17-00-async-await.html).

Complete the [backend framework prerequisite track](../00-prerequisite/README.md) before starting this project.

## Actors and tenancy

- **Client application** is a BankBridge customer with server-side credentials.
- **End user** owns or is authorized to access linked financial accounts.
- **Institution** authenticates the user and supplies account data.
- **Connector worker** refreshes tokens, polls data, and normalizes provider responses.
- **Operations and support** diagnose connections through redacted metadata and audited actions.

The ownership chain is `client -> user -> item -> account`. Every API query must include the authenticated `client_id`; externally supplied user or account IDs are never sufficient authorization.

## Product vocabulary

- `LinkSession`: single-use, short-lived configuration for a linking attempt.
- `Item`: one user's consented connection to one institution.
- `Institution`: connector capabilities, health, regions, and branding metadata.
- `Account`: normalized account identity, type/subtype, mask, currency, and status.
- `BalanceSnapshot`: current/available/limit values with source timestamp and freshness.
- `Transaction`: normalized posted or pending activity with source provenance.
- `SyncCursor`: opaque position in the client's transaction change stream.
- `ConsentGrant`: products, accounts, purposes, granted/expiry/revoked timestamps.

## Required client API

Use secret client credentials for server endpoints. Return a stable error envelope with `error_type`, `error_code`, `display_message`, `request_id`, and `retryable`. Timestamps are UTC RFC 3339; money is integer minor units plus currency.

| Method | Endpoint | Purpose |
| --- | --- | --- |
| `POST` | `/v1/link/sessions` | Create a single-use link session for a client user |
| `POST` | `/v1/link/exchange` | Exchange a one-time public token for an access token |
| `GET` | `/v1/institutions` | Search available institutions and capabilities |
| `GET` | `/v1/institutions/{id}/status` | Read current connector health and incident state |
| `GET` | `/v1/items/{id}` | Inspect connection, consent, and error state |
| `POST` | `/v1/items/{id}/refresh` | Request a rate-limited asynchronous refresh |
| `DELETE` | `/v1/items/{id}` | Revoke consent and schedule data/token deletion |
| `GET` | `/v1/accounts?item_id=...` | List normalized accounts and freshness metadata |
| `GET` | `/v1/accounts/{id}/balance` | Read the most recent balance snapshot |
| `POST` | `/v1/transactions/sync` | Return added, modified, and removed transactions plus next cursor |
| `POST` | `/v1/webhook_endpoints` | Register a signed client webhook destination |
| `POST` | `/v1/sandbox/items/{id}/events` | Trigger sandbox mutations and failures |

The public access token is a high-entropy opaque credential shown once. Store only a keyed hash for lookup. It is scoped to one item, one environment, and the client that exchanged it.

## Link and OAuth flow

1. The client server creates a `LinkSession` for its own stable `client_user_id`, requested products, redirect URI, and webhook URL.
2. The browser opens a minimal hosted link UI using the short-lived link token.
3. BankBridge creates an authorization request with cryptographically random `state`, nonce, and PKCE verifier.
4. The institution authenticates the user and redirects to an exact allowlisted URI.
5. BankBridge validates state once, exchanges the code server-to-server, and records the granted scope and expiry.
6. The user selects accounts and confirms consent. BankBridge issues a one-time public token.
7. The client server exchanges it for the item ID and access token.

Reject replayed callbacks and token exchanges. Bind state to client, session, institution, redirect URI, and browser session. Do not place institution tokens or BankBridge access tokens in query strings or browser storage.

## Connector contract

Define an async connector trait with capabilities similar to:

```rust
authorize_url, exchange_code, refresh_token, revoke_token,
fetch_accounts, fetch_balances, fetch_transaction_page
```

Connector outputs use provider-neutral intermediate records plus opaque provenance. Classify failures as user action required, consent expired, institution unavailable, rate limited, malformed upstream data, or permanent unsupported behavior. Preserve the upstream request ID for support without leaking tokens or personal data.

Apply per-institution concurrency and rate limits. Honor `Retry-After`, use exponential backoff with jitter, and open a circuit breaker during sustained faults. OAuth refresh must use a per-item lock and compare-and-swap token version so concurrent workers cannot overwrite a rotated refresh token.

## Data normalization rules

- Preserve institution identifiers separately from BankBridge identifiers.
- Model current, available, and credit-limit balances independently; absent is not zero.
- Store original and merchant names separately.
- Represent pending and posted transactions explicitly.
- Normalize signs consistently and document whether positive means money leaving or entering the account.
- Record provider category/code alongside any BankBridge category.
- Retain source timestamps, observed timestamps, and raw-record hashes.
- Quarantine malformed records and continue the item sync when safe; never invent required values.

Encrypt the minimal raw provider payload needed for replay/debugging under a short retention period. Normalized records remain the served source of truth.

## Incremental transaction synchronization

`POST /v1/transactions/sync` accepts an optional opaque cursor and a bounded page size. It returns `added`, `modified`, `removed`, `next_cursor`, and `has_more`.

- A cursor belongs to exactly one client user and environment.
- Cursors are authenticated opaque values or random references, not client-editable offsets.
- Repeating a request with the same cursor returns the same logical change page during its retention window.
- Apply upstream pages to a staging sync run; publish its changes and new cursor atomically only after the run completes.
- A pending transaction becoming posted normally appears as removal of the pending ID and addition of the posted ID unless the institution supplies a durable mapping.
- A provider deletion creates a tombstone. It must not silently erase audit/provenance state.
- Late updates increment the normalized record version and appear in `modified`.
- If an institution invalidates its cursor, rebuild from a safe lookback window and diff without duplicating client-visible transactions.

Document cursor expiry and force a safe restart with a typed error when history is no longer available.

## Consent, privacy, and deletion

Consent records include client, end user, institution, accounts, product scopes, purpose, policy version, and timestamps. Enforce product and account scope on every refresh and read. Expiry or revocation stops new collection immediately.

Revocation must enqueue upstream token revocation, disable access, notify the client, and delete or anonymize retained personal data according to policy. Keep only legally required minimal audit facts. Provide an idempotent deletion workflow whose progress is inspectable and recoverable after worker failure.

## Webhooks

Send versioned events such as `item.login_required`, `item.error`, `item.consent_expiring`, `transactions.updated`, and `deletion.completed`. Store an immutable event before delivery, sign it with HMAC-SHA256, retry safely, and protect delivery against SSRF. Webhooks are hints: clients use the API and cursor to retrieve authoritative changes.

## Sandbox institution

Build a deterministic institution simulator supporting OAuth, accounts, paginated transactions, refresh-token rotation, token revocation, and scripted failures. Scenarios must include:

- normal checking and credit accounts;
- pending-to-posted transaction conversion;
- modified and removed transactions;
- consent expiry and login-required recovery;
- paginated history with rate limiting;
- timeout after a successful upstream operation;
- malformed records and out-of-order webhook hints;
- institution outage and recovery.

Use a virtual clock or injectable time source so refresh, expiry, and backoff tests are deterministic.

## Security baseline

- Encrypt institution tokens and sensitive raw data with envelope encryption; record key version and support online re-encryption.
- Hash client secrets and access tokens with a keyed server-side pepper; show plaintext once.
- Store encryption keys outside the database and separate development, sandbox, and production credentials.
- Use exact redirect-URI allowlists, one-time OAuth state, PKCE, secure cookies, CSRF protection, and strict browser CSP.
- Redact account names, masks, transaction descriptions, tokens, authorization codes, and personal identifiers from logs.
- Prevent SSRF for connector and webhook traffic with destination policy, DNS/IP validation, redirect restrictions, and egress controls.
- Rate-limit by client, end user, item, and institution. Bound response sizes and pagination.
- Require audited, time-limited support access; do not build a general-purpose “view user data” admin page.
- Write a threat model covering token theft, cross-tenant access, OAuth mix-up, replay, malicious institutions, and compromised workers.

## Required implementation stack

| Concern | Required choice | How BankBridge uses it |
| --- | --- | --- |
| Async runtime | Tokio | API serving, connector jobs, bounded scheduling, cancellation, timeouts, and shutdown |
| HTTP API | Axum and Serde | Client API, hosted link callbacks, typed extractors, stable errors, and shared state |
| Middleware | Tower, `tower-http`, and `tower-governor` | Request IDs, tracing, limits, timeouts, load control, and quotas by client/institution |
| Database | PostgreSQL, SeaORM, and SQLx | SeaORM for clients/items/accounts/consent relationships; SQLx for explicit sync, cursor, locking, and outbox transactions |
| Authentication | `tower-sessions`, `axum-login`, `jsonwebtoken`, and Argon2 | Hosted-link/operator sessions, constrained bearer validation, and operator password hashing; client access tokens remain opaque hashed credentials |
| Outbound HTTP | Reqwest | OAuth and institution connectors with explicit timeouts, proxy policy, bounded responses, and controlled redirects |
| Validation and secrets | `validator` and `secrecy` | Boundary validation and redaction of client, OAuth, institution, and encryption credentials |
| Observability | `tracing`, OpenTelemetry, `metrics`, and Prometheus exporter | Cross-service trace context and low-cardinality API/connector/sync metrics |
| Tests | Tower `ServiceExt`, Reqwest, and Wiremock | In-process API tests, complete OAuth flows, and deterministic institution failures |

SeaORM must not hide the hard synchronization invariants. Use explicit SQLx transactions where a sync run must atomically publish changes and advance a cursor, where token versions require compare-and-swap, or where workers require row locks. Record the reason at each escape hatch.

## Architecture

Use a Cargo workspace with `bankbridge-domain`, `bankbridge-api`, `bankbridge-store`, `bankbridge-connectors`, `bankbridge-worker`, `bankbridge-webhook`, and `bankbridge-sandbox`. Keep normalized domain types separate from Axum extractors, SeaORM entities, SQLx rows, and provider DTOs.

PostgreSQL is authoritative for items, consent, normalized data, sync runs, jobs, and an outbox. Run API and workers separately. A queue may wake workers, but correctness must survive lost or duplicate queue messages.

## Required tests

### Unit and property tests

- `T01` — Money/sign normalization covers debit, credit, reversal, and missing currency cases.
- `T02` — OAuth state validation rejects wrong client, session, redirect URI, expiry, and replay.
- `T03` — Generated transaction change sequences converge to the expected materialized state.
- `T04` — Cursor encoding rejects tampering and cross-user reuse.
- `T05` — Failure classification maps connector errors to stable client errors and safe retry policy.

### Integration tests

- `T06` — Complete link, callback, public-token exchange, account read, and first sync against the sandbox.
- `T07` — Concurrent refresh workers do not lose a rotated refresh token or perform duplicate committed syncs.
- `T08` — The same cursor returns a stable page, and following all pages advances exactly once.
- `T09` — Pending-to-posted, modified, and removed transactions produce the documented change sets.
- `T10` — Client A cannot use IDs, access tokens, or cursors belonging to client B.
- `T11` — Revocation blocks reads and refreshes immediately and resumes deletion after process restart.

### End-to-end and resilience tests

- `T12` — Link an item, sync data, rotate an upstream token, expire consent, reauthorize, and continue syncing.
- `T13` — Inject `429`, timeouts, malformed pages, duplicate hints, and an outage without losing prior good data.
- `T14` — Kill workers at each sync checkpoint and verify atomic visibility and cursor recovery.
- `T15` — Rotate the data-encryption key and prove old and newly written tokens remain decryptable only through the key service.
- `T16` — Deliver signed webhooks with retries and prove no private-network destination is reached.
- `T17` — Load test a realistic mix of link, read, and scheduled sync operations within institution rate limits.

## Operations and observability

Emit redacted logs and traces with request, client, item, institution, sync-run, and connector-request IDs. Measure API rate/errors/duration, institution success and latency, `429` rate, circuit state, item error counts, token-refresh failures, sync lag, queue age, cursor invalidations, quarantined records, webhook backlog, deletion age, and encryption-key versions.

Provide runbooks for an institution outage, mass consent expiry, compromised client secret, connector schema change, stuck deletion, lost encryption-key access, sync divergence, database restore, and rollback. Expose an institution status page derived from measured connector health rather than manual optimism.

## Service objectives

- Core read API availability: `99.9%` monthly in the exercise environment, excluding a specific institution's outage from its connector SLO.
- p95 cached account and transaction API latency: under `300 ms` at declared load.
- Scheduled sandbox items refreshed within `15 minutes` p99 when the institution is healthy.
- No cross-client disclosure and no plaintext sensitive token in logs, traces, or database columns.
- Revoked consent blocks new collection within `60 seconds` and immediately blocks client reads.
- No acknowledged sync cursor points past uncommitted transaction changes.

## Deployment and deliverables

Provide non-root OCI images, versioned migrations, a local Compose environment, sandbox seed data, health/readiness endpoints, configuration reference, backup/restore procedures, encryption-key rotation procedure, dashboards, alerts, and rollback instructions. Include OpenAPI, connector authoring guide, data dictionary, consent/deletion policy, threat model, architecture decisions, and an incident-response checklist.

## Acceptance criteria

The project is complete when a reviewer can link and reauthorize an item, observe realistic transaction mutations through cursor sync, survive rate limits and worker crashes, verify tenant isolation, revoke consent and complete deletion, rotate secrets and encryption keys, diagnose an institution outage from telemetry, restore a backup, and roll back a release using only the supplied documentation.

## Optional extensions

Add identity and ownership verification, liabilities, recurring-transaction detection, institution webhooks, multi-region connector workers, client usage billing, or a connector certification harness. Each new product must extend consent scope and deletion behavior explicitly.
