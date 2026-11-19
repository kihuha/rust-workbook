# 01 — Search and observability: SignalWorks metrics platform

## Business scenario

Engineering teams need metrics during incidents, precisely when traffic and failures create the
largest telemetry burst. Unbounded labels can make the monitoring system itself unavailable.

Build `signalworks`, a Rust ingest API, durable pipeline, time-series query service, rule evaluator,
and operator tooling. Reuse [PulseStore](../../08-databases/01-pulsestore/README.md) or a documented
external store so the platform lesson remains visible.

## Problem statement

Accept and query useful telemetry during incident-shaped bursts without allowing label cardinality,
wide queries, slow storage, or one tenant to exhaust the monitoring system.

## Learning objectives

- implement bounded metrics admission, storage, query, rollup, and alert evaluation;
- control series cardinality and isolate tenant ingestion/query work;
- define late/duplicate sample and partial-query behavior;
- measure freshness, bytes/sample, alert delay, overload, and rebuild.

## Prerequisites and research

Complete [parallel log analyzer](../../../projects/medium/02-parallel-log-analyzer.md),
[PulseStore](../../08-databases/01-pulsestore/README.md), or [EventLog](../../11-messaging-streaming/01-eventlog/README.md).
Review [Brown Rust Book Chapters 13, 16, and 17](https://rust-book.cs.brown.edu/ch13-00-functional-features.html),
the [Prometheus data model](https://prometheus.io/docs/concepts/data_model/), its
[storage documentation](https://prometheus.io/docs/prometheus/latest/storage/), and
[Google SRE monitoring guidance](https://sre.google/sre-book/monitoring-distributed-systems/).

## System boundaries and constraints

Implement metrics rather than logs and traces. Notification receivers and storage nodes may be
simulated. A full PromQL clone, production paging integration, and unlimited label schemas are out.

## Requirements and guarantees

- Ingest timestamped samples with metric name and bounded labels.
- Enforce tenant, series, label, sample-rate, and out-of-order policies at admission.
- Query ranges with filters and aggregations under explicit time/memory budgets.
- Create rollups/downsampling and apply retention without changing defined results.
- Evaluate alert rules with `for` duration, missing-data policy, and deduplicated notifications.
- Isolate tenant queries and ingestion; expose cardinality and cost attribution.
- Rebuild derived indexes/rollups from retained authoritative data.

Define duplicate and late-sample semantics. An acknowledged sample satisfies the stated durability
policy. Alert transitions are replay-safe; notification delivery may be at least once.

## Workload and decisions

Simulate 100 tenants, 1 million active series scaled to the machine, 10-second scrapes, cardinality
spikes, wide queries, and incident-time ingestion bursts. Measure samples/s, ingest and query
p50/p95/p99, ingestion lag, active series, bytes/sample, compaction/rollup debt, alert delay, and
fairness. Compare push/pull ingestion, raw/rollup query planning, exact/approximate aggregation, and
reject/drop/sample overload policies.

## Milestones

### Senior — bounded single-cluster metrics service

Implement admission, durable ingestion, range query, retention, rule evaluation, telemetry, and
recovery against a declared workload.

### Staff — distributed ingestion and query

Add partitioning, replication, query fan-out, rollups, tenant fairness, cardinality protection,
partial-result policy, and compatible rule/storage changes.

### Principal — observability platform

Derive possible tiered service objectives from user journeys, regional failure behavior, monitoring of the monitoring system, cost per tenant,
ownership, retention governance, and a migration between storage/query architectures.

## Required experiments

Inject cardinality explosion, slow storage, out-of-order data, lost notifications, query fan-out
failure, and one overloaded tenant. Demonstrate bounded resource use, explicit partial/failure
responses, alert recovery, rebuild, staged schema rollout, and rollback. Provide runbooks for each.

## Operational readiness

Expose accepted/rejected samples, cardinality, ingest/compaction/rollup lag, query budgets and
partials, rule evaluation, notification state, and tenant cost. Exercise storage recovery, index/
rollup rebuild, bad rule/schema rollback, cardinality response, and monitoring-system degradation.

## Completion evidence and review

Submit ingest/query/rule services, workload and incident harnesses, correctness and rebuild tests,
benchmark/cardinality/cost report, threat model, ADRs, dashboards, alerts, and runbooks. Enforce
tenant isolation and label/query limits; protect telemetry and alert destinations. Defend sample
semantics, partial queries, cardinality policy, alert reliability, retention economics, and how the
monitoring system is observed during its own failure.

## Architecture review questions

- Which data is authoritative, derived, downsampled, or safe to rebuild?
- What is rejected or degraded first during an incident-shaped telemetry burst?
- How are missing data and partial queries reflected in alert correctness?
