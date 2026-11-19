# System-design curriculum

This path turns Rust knowledge into architectural judgment through implementation. The unit of
learning is not “know what Kafka does”; it is “build enough of a durable log to observe ordering,
replay, backpressure, failure recovery, and the cost of each guarantee.”

## Three workbook layers

| Layer | Purpose | Evidence |
| --- | --- | --- |
| [`chapters/`](../../chapters/) | Learn one Rust concept at a time | Small exercises and focused tests |
| [`projects/`](../../projects/README.md) | Integrate concepts behind clean boundaries | Working local programs, invariants, and failure tests |
| [`real-world-usage/`](../../real-world-usage/README.md) | Solve ambiguous production and system-design problems | ADRs, implementation, measurements, fault injection, runbooks, and design defense |

Do not skip the implementation layer. A design is not understood until you can point to the state
transition, bounded queue, transaction, recovery path, or measurement that supports the claim.

## Recommended portfolio

The order is a dependency guide, not a gate. Each real-world brief may use an existing database or
queue when rebuilding it would distract from the current lesson.

| Phase | Build | Main judgment developed |
| --- | --- | --- |
| 0 — Rust integration | Two easy, two medium, and two hard [projects](../../projects/README.md) | Ownership, modeling, boundaries, testing, persistence, and concurrency |
| 1 — Service foundations | [StreamGate](../../real-world-usage/05-networking/01-streamgate/README.md), [SwiftCart](../../real-world-usage/01-backend-development/01-swiftcart/README.md), [DurableQueue](../../real-world-usage/02-cloud-infrastructure/05-durablequeue/README.md) | APIs, latency, transactions, backpressure, retries, and recovery |
| 2 — Distributed foundations | [ShardStore](../../real-world-usage/08-databases/04-shardstore/README.md), [RaftKeeper](../../real-world-usage/02-cloud-infrastructure/06-raftkeeper/README.md), [EventLog](../../real-world-usage/11-messaging-streaming/01-eventlog/README.md) | Partitioning, replication, consensus, ordering, replay, and consistency |
| 3 — Product platforms | [PayForge](../../real-world-usage/01-backend-development/02-payforge-stripe-clone/README.md), [CollabPad](../../real-world-usage/12-real-time-collaboration/01-collabpad/README.md), [SignalWorks](../../real-world-usage/13-search-observability/01-signalworks/README.md) | Business invariants, synchronization, ingestion, querying, and operations |
| 4 — Staff/Principal | [TenantPlane](../../real-world-usage/02-cloud-infrastructure/07-tenantplane/README.md), [RegionGate](../../real-world-usage/02-cloud-infrastructure/08-regiongate/README.md), [SearchGrid](../../real-world-usage/13-search-observability/02-searchgrid/README.md) | Isolation, fairness, failover, migration, ownership, and cost |
| 5 — Specialization | [ModelGate](../../real-world-usage/14-ai-infrastructure/01-modelgate/README.md) | Streaming inference, routing, evaluation, quotas, and provider economics |

## Competency map

| Category | First practical foundation | Culminating brief |
| --- | --- | --- |
| Scalable services | [Rate-limited HTTP service](../../projects/hard/07-rate-limited-http-service.md) | [StreamGate](../../real-world-usage/05-networking/01-streamgate/README.md) |
| Transaction correctness | [Event booking engine](../../projects/medium/09-event-booking-engine.md) | [SwiftCart](../../real-world-usage/01-backend-development/01-swiftcart/README.md) and [PayForge](../../real-world-usage/01-backend-development/02-payforge-stripe-clone/README.md) |
| Durable asynchronous work | [Durable job scheduler](../../projects/hard/03-durable-job-scheduler.md) | [DurableQueue](../../real-world-usage/02-cloud-infrastructure/05-durablequeue/README.md) |
| Storage | [Persistent KV store](../../projects/hard/02-persistent-key-value-store.md) | [LogStore](../../real-world-usage/08-databases/03-logstore/README.md) then [ShardStore](../../real-world-usage/08-databases/04-shardstore/README.md) |
| Coordination | [Configuration merger](../../projects/medium/07-configuration-merger.md) and scheduler leases | [RaftKeeper](../../real-world-usage/02-cloud-infrastructure/06-raftkeeper/README.md) |
| Streaming | [Parallel log analyzer](../../projects/medium/02-parallel-log-analyzer.md) | [EventLog](../../real-world-usage/11-messaging-streaming/01-eventlog/README.md) |
| Real-time systems | [Async chat server](../../projects/hard/01-async-chat-server.md) | [CollabPad](../../real-world-usage/12-real-time-collaboration/01-collabpad/README.md) |
| Search | [Local note index](../../projects/medium/10-local-note-index.md) | [SearchGrid](../../real-world-usage/13-search-observability/02-searchgrid/README.md) |
| Metrics and analytics | [Temperature journal](../../projects/easy/05-temperature-journal.md) and [PulseStore](../../real-world-usage/08-databases/01-pulsestore/README.md) | [SignalWorks](../../real-world-usage/13-search-observability/01-signalworks/README.md) |
| Multi-tenancy | Repository and API isolation exercises | [TenantPlane](../../real-world-usage/02-cloud-infrastructure/07-tenantplane/README.md) |
| Multi-region | [NimbusFleet](../../real-world-usage/02-cloud-infrastructure/01-nimbusfleet/README.md) | [RegionGate](../../real-world-usage/02-cloud-infrastructure/08-regiongate/README.md) |
| AI infrastructure | StreamGate, EventLog, TenantPlane, and SearchGrid | [ModelGate](../../real-world-usage/14-ai-infrastructure/01-modelgate/README.md) |

## Senior, Staff, and Principal progression

- **Senior — make it correct:** implement the end-to-end system, its contracts and invariants;
  benchmark a stated workload; recover from expected failures.
- **Staff — make it resilient:** introduce contention, partial failure, multiple components,
  saturation, compatibility, observability, and quantified alternatives.
- **Principal — evolve the platform:** introduce regions, tenants, budgets, conflicting goals,
  ownership boundaries, staged migrations, verification, and rollback.

Principal does not mean “add more services.” Prefer a simpler system when it satisfies the stated
requirements with lower operational and organizational cost.

## Required loop

1. Clarify the ambiguous brief and record assumptions.
2. Estimate load, storage, bandwidth, and failure budgets.
3. Write ADRs before implementation commits to hard-to-reverse choices.
4. Build the smallest end-to-end milestone in Rust.
5. Test invariants and recovery, then benchmark the declared workload.
6. Inject overload, latency, crashes, and dependency failures.
7. Add the next requirement envelope and migrate safely.
8. Defend the design using the [evaluation rubric](evaluation-rubric.md).

Use the [project-brief standard](project-brief-standard.md) when adding or reviewing briefs and the
[research guide](../system-design-reading-guide.md) for background.
