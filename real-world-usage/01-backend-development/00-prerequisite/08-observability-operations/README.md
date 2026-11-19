# 08 — Observability and operations

## Goal

Instrument a small Axum API and worker so an operator can follow one request through PostgreSQL and an outbound connector, identify saturation, and shut the service down safely—without leaking secrets or creating unbounded telemetry.

Use [`tracing`](https://docs.rs/tracing/latest/tracing/), [`tracing-subscriber`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/), [OpenTelemetry Rust](https://opentelemetry.io/docs/languages/rust/), [`tracing-opentelemetry`](https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/), [`metrics`](https://docs.rs/metrics/latest/metrics/), [Prometheus exporter](https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/), [`config`](https://docs.rs/config/latest/config/), and [`secrecy`](https://docs.rs/secrecy/latest/secrecy/).

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
