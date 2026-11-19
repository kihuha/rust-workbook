# Production system-design project-brief standard

Cover these content areas in every new real-world brief and when substantially revising an existing
one. Closely related areas may share a heading when that makes a brief easier to use; the content and
completion evidence remain mandatory.

## Required content

1. **Business scenario** — who needs the system, why it matters, and consequences of failure.
2. **Problem statement** — observable outcome without prescribing the solution.
3. **Learning objectives** — three to five architectural skills demonstrated by implementation.
4. **Functional requirements** — externally visible behavior and contracts.
5. **Workload and capacity** — traffic mix, concurrency, sizes, skew, growth, and estimates.
6. **Correctness and consistency** — invariants, ordering, durability, and allowed stale results.
7. **Boundaries and constraints** — what is in scope, fixed, simulated, or deliberately excluded.
8. **Decisions to explore** — at least two plausible alternatives without revealing an answer.
9. **Progressive milestones** — independently demonstrable Senior, Staff, and Principal envelopes.
10. **Failure injection** — crashes, latency, partitions, malformed input, overload, and recovery.
11. **Measurement plan** — workload generator, environment, metrics, and saturation behavior.
12. **Acceptance criteria** — executable evidence, never “scales well” or “is highly available.”
13. **Operational readiness** — local measurement targets, dashboards, alerts, deployment, rollback,
    and runbooks. Production SLOs are a design exercise, not a claim about a local clone.
14. **Security and abuse** — trust boundaries, authorization, secrets, quotas, and misuse cases.
15. **Architecture review** — alternatives, bottlenecks, cost, migration, and rejected guarantees.
16. **Research and prerequisites** — Brown Rust Book links plus authoritative domain sources.

## Measurement rule

Every latency or throughput target must name its workload. Record CPU, memory, disk, network,
compiler profile, dataset, cache state, test duration, and machine. Report p50/p95/p99 rather than
only averages. Run past the intended operating point and document how the system saturates.

Local tests establish correctness and expose trends; they do not prove internet-scale capacity.
Do not assign fictional monthly availability to an educational service. If SLO design is relevant,
derive the SLI, population, objective, window, error budget, dependencies, and response policy, and
state that the repository has not operated the service long enough to validate the objective.

## Architecture rule

Separate fixed learning constraints from open choices. A brief may require Rust, a wire contract,
or a particular primitive when that is the lesson. It should not prescribe every crate, topology,
or storage choice before the learner can compare alternatives.

## Milestone rule

Each milestone must run and be reviewable on its own. Later milestones should introduce changed
requirements and require evolution of a running design rather than a clean rewrite.
