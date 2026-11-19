# 07 — Outbound integrations and reliability

## Goal

Build `parcel-connector`, an adapter to an unreliable carrier API. The local API creates shipment operations and a worker reconciles ambiguous outcomes without creating duplicate labels.

Use [Reqwest](https://docs.rs/reqwest/latest/reqwest/), [Wiremock](https://docs.rs/wiremock/latest/wiremock/), and [Tower retry](https://docs.rs/tower/latest/tower/retry/trait.Policy.html). Reuse Tokio cancellation, SQLx transactions, and Tower traffic-control knowledge.

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
