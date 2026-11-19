# System-design implementation evaluation rubric

| Dimension | Weight | Evidence |
| --- | ---: | --- |
| Correctness and consistency | 20% | Invariants, concurrent behavior, recovery, and explicit guarantees |
| Architecture and trade-offs | 20% | Boundaries, viable alternatives, ADRs, and justified simplicity |
| Performance and scalability | 15% | Capacity estimates, reproducible workload, bottlenecks, and measurements |
| Reliability and resilience | 15% | Fault injection, backpressure, degradation, and recovery evidence |
| Operations and observability | 10% | Useful telemetry, honest local targets, alerts, deployment, rollback, and runbooks; any proposed SLO is derived rather than claimed |
| Security and isolation | 10% | Trust boundaries, authorization, abuse resistance, and tenant isolation |
| Cost and evolution | 10% | Resource economics, compatibility, migration, and operational burden |

Score the evidence, not the number of components. A smaller design with measured behavior and a
safe migration plan should beat an elaborate design supported only by claims.

## Required design defense

The learner must explain assumptions, rejected alternatives, saturation behavior, tolerated and
untolerated failures, deliberately omitted guarantees, 10x and 100x changes, production gaps, and
the decisions that are hardest to reverse.

For Principal practice, separately review product constraints, organizational ownership, rollout
strategy, recovery investment, and long-term architectural consequences.
