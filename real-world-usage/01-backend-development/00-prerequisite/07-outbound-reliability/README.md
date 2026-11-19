# 07 — Outbound integrations and reliability

## Goal

Build `parcel-connector`, an adapter to an unreliable carrier API. The local API creates shipment operations and a worker reconciles ambiguous outcomes without creating duplicate labels.

Use [Reqwest](https://docs.rs/reqwest/latest/reqwest/), [Wiremock](https://docs.rs/wiremock/latest/wiremock/), and [Tower retry](https://docs.rs/tower/latest/tower/retry/trait.Policy.html). Reuse Tokio cancellation, SQLx transactions, and Tower traffic-control knowledge.

## Before you start

Complete challenge 04's transaction/idempotency milestones and challenge 01's cancellation milestone. You need to understand HTTP requests and status codes but not carrier APIs. Build this as a separate `parcel-connector` service with a tiny local operations table; Wiremock is the carrier.

Do not begin with retries. First make one attempt observable and correctly classified. A retry loop around an unclassified error is how duplicate external effects are created.

## Scenario and provider contract

Your fictional carrier supports:

- `POST /labels` with `X-Operation-Id`, returning `201`, a safe `4xx`, `429`, or an ambiguous connection/timeout failure;
- `GET /operations/{operation_id}` returning pending, succeeded with a label, failed, or not found; and
- `DELETE /labels/{label_id}` as an idempotent cancellation.

Your local API accepts a label request and returns a local operation ID. A worker, not the request handler, calls the carrier. Keep provider JSON types and error codes inside the adapter.

## Mental model

A timeout tells you that the client stopped waiting; it does not prove the server did nothing. Separate **admission** (did this service accept durable responsibility?), **attempt** (was a request sent?), and **outcome** (is the external effect known?). Retry only when semantics or an idempotency key make replay safe. Use reconciliation to resolve ambiguity.

## Build it in milestones

### 1. Map one successful call

Define a domain-facing connector trait and provider-specific Reqwest adapter. Configure the base URL and authentication; use one shared client. Make Wiremock return a successful label and map it into a domain result.

**Checkpoint:** the domain/application test knows no provider DTO or URL, and Wiremock verifies the operation ID and redacted authentication header were sent.

### 2. Classify every single-attempt result

Add the result classes in the contract below. Bound response size and parse failures. Configure connect and per-request deadlines; keep the overall operation deadline in the calling workflow.

**Checkpoint:** table-driven Wiremock tests cover success, safe rejection, `429`, `5xx`, malformed/truncated/oversized body, and timeout without retrying any of them yet.

### 3. Persist before dispatch

Store an accepted local operation and stable provider operation ID in one transaction before making the call. Model states such as `pending`, `in_flight`, `reconciling`, `succeeded`, and `failed`, with attempts and lease expiry.

**Checkpoint:** killing/canceling after persistence but before dispatch leaves work a restarted worker can claim.

### 4. Resolve ambiguity

When create times out or the connection drops after sending, mark the result unknown and call provider lookup. Never immediately replay create. Apply a documented not-found grace period because lookup data may lag creation.

**Checkpoint:** Wiremock simulates “created then response lost”; reconciliation records the one label and no second `POST` occurs.

### 5. Add bounded retry and pressure policy

Retry safe reads and provider-confirmed idempotent operations for a bounded number of attempts and total time. Honor bounded `Retry-After`; otherwise use exponential backoff plus jitter. Limit in-flight requests per provider. Then add a simple circuit with a small closed/open/half-open state machine.

**Checkpoint:** paused-time tests prove the schedule and budget; the open circuit rejects promptly and permits only the documented recovery probe.

### 6. Harden destinations and shutdown

Allow configured HTTPS origins only, reject cross-host credential redirects, and define proxy behavior. On shutdown stop claiming work, persist enough state to resume, let safe in-flight attempts reach a deadline, then join workers.

**Checkpoint:** a redirect test cannot send credentials to a second mock host, and every accepted operation reaches a resumable terminal/nonterminal state after restart.

## Connector contract

Define a domain-facing async trait for create-label, lookup-operation, and cancel-label. Keep provider DTOs, authentication, error codes, pagination, and retry headers inside the adapter.

Classify results as success, safe client rejection, rate limited, transient unavailable, ambiguous timeout, malformed upstream response, or permanent provider incompatibility.

## Reliability requirements

- Configure separate DNS/connect, request, and overall operation deadlines.
- Bound response bodies and connection-pool resources.
- Send a stable provider operation ID for every create command.
- Retry only operations proven safe by HTTP semantics or provider idempotency.
- Honor bounded `Retry-After`; otherwise use exponential backoff with jitter and a retry budget.
- Persist operation state before dispatch and reconcile timeouts by lookup rather than blind replay.
- Limit concurrency per provider and open a simple circuit after a documented failure threshold.
- Distinguish admission rejection from an already-started operation whose result is unknown.
- Propagate cancellation while preserving enough durable state for a later worker to resume.

## Security requirements

Validate outbound destinations against configuration. Disable uncontrolled redirects, prevent credentials from crossing hosts, redact headers/bodies, constrain proxy use, and document SSRF controls for any user-influenced URL.

## Required tests

These are graduation tests after the milestone checkpoints pass.

- `T01` — A successful create maps provider data into domain types without leaking provider DTOs.
- `T02` — Safe reads retry transient failures with deterministic paused-time backoff.
- `T03` — A create timeout triggers lookup and does not create a second label.
- `T04` — `429` honors a bounded `Retry-After` and consumes the retry budget.
- `T05` — Malformed, oversized, and truncated responses return bounded typed errors.
- `T06` — The circuit opens, rejects promptly, probes recovery, and closes after success.
- `T07` — Cancellation and process restart resume every accepted operation from durable state.
- `T08` — Redirect and destination tests prove credentials cannot reach an unapproved host.

## Completion evidence

Submit a failure matrix, retry decision table, connector trait, Wiremock scenarios, and a reconciliation runbook. You should be able to explain why timeout is not proof of failure and why generic automatic retry is dangerous for money or other non-idempotent effects.
