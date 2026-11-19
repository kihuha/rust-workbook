# System-design research guide

Use these sources when a project introduces unfamiliar context. Read only what supports the next
design or experiment, record the conclusion in an ADR, and verify the claim in Rust where the local
environment permits it.

## Rust foundations

- [Brown interactive Rust Book](https://rust-book.cs.brown.edu/) — the workbook's primary language reference.
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/) — futures, executors, cancellation, and pinning.
- [Tokio tutorial](https://tokio.rs/tokio/tutorial) — practical async networking, channels, I/O, and task management.
- [Tower guides](https://docs.rs/tower/latest/tower/) — service composition, readiness, load shedding, limits, and retries.
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/) — profiling and evidence-led optimization.
- [The Cargo Book: workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) — multi-crate boundaries.

## Distributed systems and storage

- [Designing Data-Intensive Applications resources](https://dataintensive.net/) — storage, replication, partitioning, transactions, streams, and derived data.
- [Raft paper](https://raft.github.io/raft.pdf) and [Raft site](https://raft.github.io/) — consensus, terms, log replication, and membership.
- [Dynamo paper](https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf) — partitioning, sloppy quorums, hinted handoff, and reconciliation.
- [Google Spanner paper](https://research.google/pubs/spanner-googles-globally-distributed-database/) — global transactions and the cost of strong guarantees.
- [PostgreSQL transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html) — concrete isolation semantics and anomalies.
- [RocksDB wiki](https://github.com/facebook/rocksdb/wiki) — operational consequences of an LSM implementation.

## Messaging and streaming

- [Kafka design documentation](https://kafka.apache.org/documentation/#design) — logs, partitions, replication, consumer position, delivery semantics, and logs as a data-integration primitive.
- [Reactive Streams specification](https://www.reactive-streams.org/) — asynchronous demand and backpressure.

## Reliability and operations

- [Google SRE books](https://sre.google/books/) — SLOs, overload, cascading failures, monitoring, and incident response.
- [AWS Well-Architected Framework](https://docs.aws.amazon.com/wellarchitected/latest/framework/welcome.html) — operations, security, reliability, performance, cost, and sustainability review questions.
- [OpenTelemetry concepts](https://opentelemetry.io/docs/concepts/) — traces, metrics, logs, context, and semantic conventions.
- [OpenSLO specification](https://openslo.com/) — portable vocabulary for service objectives.

## Security

- [OWASP API Security Top 10](https://owasp.org/API-Security/) — API authorization, resource consumption, SSRF, and unsafe dependencies.
- [OWASP Threat Modeling](https://owasp.org/www-community/Threat_Modeling) — assets, trust boundaries, threats, and mitigations.
- [NIST Secure Software Development Framework](https://csrc.nist.gov/Projects/ssdf) — secure development and supply-chain practices.

## Collaboration, search, and analytics

- [CRDT survey](https://inria.hal.science/hal-00932836/document) — convergence models and CRDT design space.
- [Apache Lucene documentation](https://lucene.apache.org/core/) — inverted indexes, segments, scoring, and query execution.
- [Prometheus data model](https://prometheus.io/docs/concepts/data_model/) and [storage](https://prometheus.io/docs/prometheus/latest/storage/) — labels, cardinality, and time-series trade-offs.

## AI infrastructure

- [vLLM documentation](https://docs.vllm.ai/) — serving, batching, scheduling, and operational metrics.
- [OpenAI production best practices](https://platform.openai.com/docs/guides/production-best-practices) — request limits, latency, cost, and operational controls.
- [NIST AI Risk Management Framework](https://www.nist.gov/itl/ai-risk-management-framework) — evaluation, risk, and governance context.

Vendor documentation describes a particular system, not a universal design. Compare guarantees,
failure assumptions, and workload before transferring a technique into a project.
