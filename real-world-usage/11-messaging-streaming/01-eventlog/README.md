# 01 — Messaging and streaming: EventLog platform

## Business scenario

Product teams publish account, billing, and activity events. Fraud, analytics, and notification
consumers need independent replayable histories. Producers exceed consumer speed during bursts,
and a consumer restart must not silently skip accepted data.

Build `eventlog`, a Rust broker, producer/consumer clients, and operator CLI. Its purpose is to
expose partitions, offsets, retention, rebalance, and delivery semantics through implementation.

## Problem statement

Retain accepted ordered histories for independent consumers, bound producer and consumer lag, and
recover position safely without pretending external effects become exactly once automatically.

## Learning objectives

- implement a durable partition log, offsets, retention, and recovery;
- coordinate consumer ownership with generation fencing;
- explain and demonstrate delivery and ordering guarantees;
- measure lag, rebalance, replication, hot partitions, and saturation.

## Prerequisites and research

Complete [DurableQueue](../../02-cloud-infrastructure/05-durablequeue/README.md) and preferably
[LogStore](../../08-databases/03-logstore/README.md). Review [Brown Rust Book Chapters 16–17](https://rust-book.cs.brown.edu/ch16-00-concurrency.html),
[Kafka design](https://kafka.apache.org/documentation/#design),
and the [research guide](../../../research/system-design-reading-guide.md).

## System boundaries and constraints

Support a narrow record protocol and small local cluster. General stream SQL, connectors, arbitrary
transactions, geo-mirroring, and a production schema registry are later platform exercises.

## Requirements and guarantees

- Create topics with partition count, retention, and payload limits.
- Append keyed records and return partition, offset, and durability acknowledgement.
- Fetch bounded batches from an offset and expose high-water marks.
- Track consumer-group membership and committed offsets with generation fencing.
- Reassign partitions after member change without concurrent active ownership in one generation.
- Retain/reclaim segments without deleting data still inside the configured policy.
- Apply producer and tenant quotas; reject before unbounded buffers form.

Guarantee ordering only within a partition. State when an acknowledged append survives failure.
Consumer offset commit and external side effects are not automatically atomic; demonstrate
at-least-once processing and one safe effect pattern such as idempotency or a transactional outbox.

## Workload and decisions

Run 100 producers, 10 groups, 12 partitions, records from 100 B to 64 KiB, one hot key, and a
consumer that falls 10 minutes behind. Report append/fetch p50/p95/p99, records/bytes per second,
lag, rebalance pause, disk and page-cache behavior, recovery, and rejection at saturation.
Compare push/pull, per-record/batched flush, static/dynamic membership, eager/cooperative
rebalance, and replicated versus local-only durability.

## Milestones

### Senior — single-node log

Implement segments, checksums, indexes, recovery, bounded append/fetch, retention, client offsets,
and restart tests.

### Staff — consumer groups and replication

Add group coordination, generation fencing, partition replication, leader failover, lag-aware
backpressure, quotas, rolling compatibility, and recovery of a slow replica.

### Principal — shared event platform

Define tenant isolation, schemas and compatibility, regional placement, cost per GiB ingress and
retention, ownership, disaster recovery, and migration of a live topic between policies/clusters.

## Required experiments

- Crash at every append/flush boundary and verify the advertised acknowledgement contract.
- Kill a consumer after its external effect but before offset commit and show safe redelivery.
- Rebalance during load and prove generation fencing prevents stale commits.
- Pause a consumer until retention pressure and document the chosen outcome.
- Saturate one tenant and hot partition without unbounded memory or global starvation.
- Fail a partition leader and measure unavailability, data loss boundary, and catch-up.

Provide schema/consumer-lag dashboards, alerts, and runbooks for under-replication, disk pressure,
lag, poison records, rebalance storms, failed migration, and rollback.

## Operational readiness

Expose partition leadership, under-replication, append/fetch latency, disk, consumer lag, group
generation, rebalance, quota, and retention risk. Exercise broker replacement, replica recovery,
segment repair, group reset, protocol upgrade, live topic move, and rollback.

## Completion evidence and review

Submit broker and clients, versioned protocol/storage notes, workload and fault harnesses,
delivery/ordering tests, benchmark report, threat model, ADRs, dashboards, alerts, and runbooks.
Authenticate producers/consumers, authorize topics and groups, enforce tenant quotas, and redact
payloads. Defend partition choice, acknowledgement policy, group protocol, external-effect safety,
retention cost, and live topic migration.

## Architecture review questions

- Which acknowledgement loses what under each broker failure?
- Why is the partition key compatible with ordering and scaling needs?
- How does a consumer make external effects safe across redelivery?
