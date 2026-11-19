# Real-world Rust challenges

These briefs are the culmination of the workbook. Chapters teach individual Rust concepts, and
[`projects/`](../projects/README.md) combines those concepts into bounded programs. Here the learner
receives an incomplete production problem, must make and defend architectural decisions, implements
the important paths in Rust, and then measures and breaks the result.

Start with the [system-design curriculum](../research/system-design-curriculum/README.md). Use the
[research guide](../research/system-design-reading-guide.md) when a brief names an unfamiliar
algorithm, reliability technique, or operational practice. Briefs intentionally link to the
[Brown Rust Book](https://rust-book.cs.brown.edu/) and authoritative technical sources; reading is
preparation for an experiment, not a substitute for implementing it.

## Numbered use cases

| No. | Use case | Production challenge |
| --- | --- | --- |
| 01 | [Backend development](01-backend-development/README.md) | Order, payment-processing, and open-banking APIs |
| 02 | [Cloud infrastructure](02-cloud-infrastructure/README.md) | Regional container deployment controller |
| 03 | [CLI tools](03-cli-tools/README.md) | Reproducible monorepo release tool |
| 04 | [Systems programming](04-systems-programming/README.md) | Linux service supervisor and health daemon |
| 05 | [Networking](05-networking/README.md) | Layer-7 reverse proxy and load balancer |
| 06 | [Cybersecurity](06-cybersecurity/README.md) | Enterprise secret-scanning platform |
| 07 | [Embedded systems](07-embedded-systems/README.md) | Fault-tolerant irrigation controller |
| 08 | [Databases](08-databases/README.md) | Time-series, B+ tree, and LSM storage engines |
| 09 | [Blockchain](09-blockchain/README.md) | Supply-chain provenance registry and indexer |
| 10 | [WebAssembly](10-webassembly/README.md) | In-browser analytics engine for large CSV files |
| 11 | [Messaging and streaming](11-messaging-streaming/README.md) | Durable log, replay, consumer groups, and delivery semantics |
| 12 | [Real-time collaboration](12-real-time-collaboration/README.md) | Presence, fan-out, reconnect, and convergent editing |
| 13 | [Search and observability](13-search-observability/README.md) | Metrics ingestion, indexing, querying, alerting, and search |
| 14 | [AI infrastructure](14-ai-infrastructure/README.md) | Multi-provider inference gateway, quotas, routing, and cost |

## Delivery standard

Every new brief and substantive revision is reviewed against the
[project-brief standard](../research/system-design-curriculum/project-brief-standard.md). Compact
briefs may combine closely related headings, but they may not omit the required evidence.
At minimum, submit:

- a workload model and back-of-the-envelope capacity estimate;
- explicit correctness invariants and a stated consistency contract;
- architecture notes or ADRs that compare at least two viable approaches;
- automated correctness, concurrency, and recovery tests;
- a reproducible benchmark with p50, p95, and p99 latency where applicable;
- overload and failure-injection evidence;
- a threat model and trust-boundary diagram;
- metrics, alerts, deployment and rollback instructions, and an operations runbook;
- known limitations, cost drivers, and the next migration you would expect.

Do not claim production scale from a laptop result. State the environment, explain what the
experiment proves, and identify what requires a distributed or production environment.

Company names and business scenarios are fictional. The technical constraints are intended to resemble real production work.
