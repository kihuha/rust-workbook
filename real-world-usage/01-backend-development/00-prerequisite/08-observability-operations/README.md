# 08 — Observability and operations

## Goal

Instrument a small Axum API and worker so an operator can follow one request through PostgreSQL and an outbound connector, identify saturation, and shut the service down safely—without leaking secrets or creating unbounded telemetry.

Use [`tracing`](https://docs.rs/tracing/latest/tracing/), [`tracing-subscriber`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/), [OpenTelemetry Rust](https://opentelemetry.io/docs/languages/rust/), [`tracing-opentelemetry`](https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/), [`metrics`](https://docs.rs/metrics/latest/metrics/), [Prometheus exporter](https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/), [`config`](https://docs.rs/config/latest/config/), and [`secrecy`](https://docs.rs/secrecy/latest/secrecy/).

## Before you start

Instrument a copy of `taskboard-api` with its PostgreSQL repository and one small Wiremock-backed outbound call. Challenges 03, 04, and 07 should already pass; telemetry cannot compensate for incorrect lifecycle or failure behavior. Start with local JSON logs and a Prometheus endpoint. Add OpenTelemetry only after those signals answer a real debugging question.

## Mental model

Logs describe discrete events, metrics summarize bounded numeric behavior, and traces connect work across boundaries. Health endpoints answer whether an orchestrator should keep/restart or route traffic to a process; they are not dashboards. Observability data is another output: it needs privacy, cardinality, capacity, and failure policy.

Choose three questions before adding instruments:

1. Is traffic failing, and which stable error class increased?
2. Is the service saturated at HTTP concurrency, the DB pool, or the worker queue?
3. Which stage made one test request slow?

Every initial field, metric, and span should help answer one of them.

## Build it in milestones

### 1. Structured local logs

Install one tracing subscriber in `main`, use JSON in production mode and readable output in development, and add request/root spans containing service version, request ID, method, route template, outcome, and duration. Instrument application/repository functions; do not emit request bodies or SQL parameter values.

**Checkpoint:** a captured success and failure log share a request ID, have stable field names, and contain none of several canary secrets/customer values.

### 2. Bounded metrics

Add request count/duration by method, route template, and outcome class; then pool, queue, worker, retry, and saturation signals. Write an allowlist of label names and values. IDs, raw paths/URLs, tenant names, and error strings are forbidden labels.

**Checkpoint:** send hundreds of distinct object IDs and prove the number of metric series remains bounded.

### 3. Health with explicit semantics

`/health/live` reports whether the process/event loop is functioning and does not query dependencies. `/health/ready` uses a short budget to check whether the instance can accept useful work (including migration compatibility and critical DB access). Protect `/metrics` according to your deployment model.

**Checkpoint:** disconnect PostgreSQL: readiness fails promptly, liveness remains healthy, and neither response leaks connection details.

### 4. Add trace propagation

Install a version-compatible OpenTelemetry pipeline, accept/emit W3C trace context, and create spans for handler, application, pool wait/query, queue wait, worker attempt, and outbound call. Do not create spans for every loop iteration or record high-cardinality content as attributes.

**Checkpoint:** one test request appears as a connected trace across API, database, worker, and Wiremock request; a malformed incoming trace header starts a safe new trace.

### 5. Configuration and secrets

Create typed configuration with documented defaults, file values, environment overrides, and startup validation. Separate secret values and redact their debug display. Invalid configuration should name the field and rule, never the supplied secret.

**Checkpoint:** table-driven tests cover precedence and validation; a startup error containing a known secret fails the test.

### 6. Failure drills and shutdown

Build a small dashboard around the three opening questions and alerts tied to a runbook/action. Simulate pool exhaustion, queue saturation, upstream failure, and exporter outage. Make exporters bounded/non-fatal. On shutdown stop admission, drain owned work, and flush telemetry only to a deadline.

**Checkpoint:** each drill produces the expected signal and recovery note; exporter failure does not fail business requests or grow memory without bound.

## Required instrumentation

- Emit structured JSON logs with severity, service/version, request ID, trace/span IDs, route template, outcome, and duration.
- Propagate W3C trace context through one outbound request and connect API, database, queue, worker, and connector spans.
- Record low-cardinality request, error, duration, pool, queue, worker, retry, and saturation metrics.
- Expose `/health/live`, `/health/ready`, and `/metrics` with distinct semantics and access policy.
- Allow environment-specific filtering without recompilation; protect against a verbose-log flood.
- Layer defaults, files, and environment configuration with validation at startup.
- Wrap secrets so debug formatting is redacted and plaintext access is explicit.
- Flush telemetry during graceful shutdown to a deadline without keeping the process alive forever.

## Cardinality and privacy rules

Never use user, tenant, request, object, email, URL, token, or raw error-message values as metric labels. Do not put credentials, personal content, request bodies, SQL parameters, or institution/payment details in logs or span fields. Route templates are allowed; raw paths are not.

## Required tests and drills

These are graduation checks after the milestone checkpoints pass.

- `T01` — One request produces correlated spans across handler, repository, worker, and mock upstream.
- `T02` — A redaction test sends known canary secrets and finds none in logs, spans, metrics, or errors.
- `T03` — Metric labels remain within a fixed allowlist and bounded series count under many object IDs.
- `T04` — Readiness fails for an unusable database but liveness remains healthy.
- `T05` — Queue saturation and pool exhaustion trigger actionable metrics and alerts.
- `T06` — Invalid configuration fails startup with field names but no secret values.
- `T07` — Shutdown drains requests/jobs and flushes telemetry within its deadline.
- `T08` — An exporter outage does not take down the business API or grow memory without bound.

## Completion evidence

Provide a dashboard, alert rules with runbook links, example trace, sanitized log sample, configuration reference, and shutdown timeline. Explain which signals answer “what broke,” “where,” “for whom,” and “since when,” and which data is intentionally absent for privacy.
