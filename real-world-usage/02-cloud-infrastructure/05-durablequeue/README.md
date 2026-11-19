# 05 — Cloud infrastructure: DurableQueue work service

## Business scenario

A media platform accepts thumbnail, transcript, and export jobs for many customers. Request
handlers must hand off durable responsibility quickly. Workers vary in speed, crash, and sometimes
complete an external effect before their acknowledgement reaches the queue.

Build `durablequeue`: a Rust API, scheduler, worker protocol, and inspection CLI. The objective is
not to clone a broker; it is to demonstrate backpressure, delivery semantics, recovery, and fair
resource allocation with executable evidence.

## Problem statement

Accept work quickly without losing accepted jobs, bound resource use when workers fall behind, and
make duplicate execution, fairness, and recovery behavior explicit to clients and operators.

## Learning objectives

- distinguish admission, delivery, execution, acknowledgement, and externally visible effects;
- implement leases, redelivery, bounded queues, retry budgets, and dead-letter handling;
- preserve per-queue ordering only where the contract requires it;
- measure saturation and prevent one tenant or slow worker from exhausting the service.

## Prerequisites and research

Complete the [durable job scheduler](../../../projects/hard/03-durable-job-scheduler.md). Review
[Brown Rust Book Chapters 16–17](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), the
[Tokio tutorial](https://tokio.rs/tokio/tutorial), [Reactive Streams](https://www.reactive-streams.org/),
and the messaging section of the [research guide](../../../research/system-design-reading-guide.md).

## System boundaries and constraints

Use local processes and reproducible storage/failure simulation. External job effects are simulated
behind a trait. Exactly-once effects, geo-replication, and arbitrary workflow DAGs are out of scope.

## Functional requirements

- Create queues with retention, maximum attempts, visibility timeout, and priority policy.
- Publish one job or an atomic bounded batch using a producer idempotency key.
- Long-poll for available jobs and acknowledge, reject, extend, or abandon a lease.
- Inspect job history, queue depth, oldest age, in-flight count, and dead letters.
- Stop admission before storage or memory limits become unsafe; return a retryable overload result.
- Recover accepted jobs and expired leases after an unclean restart.

## Correctness contract

An acknowledged publish survives the documented crash model. A lease has one current fencing
token; a stale worker cannot acknowledge a later delivery. At-least-once delivery is sufficient,
so handlers must make external effects idempotent or reconcile them. Priority must not cause
permanent starvation. State precisely which operations are ordered and durable.

## Workload and capacity

Use a reproducible mix: 70% 1 KiB jobs, 25% 16 KiB, 5% 256 KiB; 20 tenants with one tenant
producing 50% of traffic; 5% slow jobs; and worker crashes during 1% of deliveries. Derive local
targets from a baseline run, then test 0.5x, 1x, 1.5x, and 2x saturation. Report publish and claim
p50/p95/p99, redelivery delay, queue age, disk growth, memory, fairness, and rejection rate.

## Architecture decisions to explore

Compare polling with server push, a mutable job table with an append-only log, strict priority with
weighted fairness, fixed with renewable leases, and database transactions with a dedicated storage
engine. Record decisions in ADRs before Staff work.

## Milestones

### Senior — durable single-node queue

Implement bounded publish, claim, lease expiry, acknowledgement, retry, dead letters, restart
recovery, telemetry, and a deterministic clock. Prove duplicate execution is possible and safe.

### Staff — replicated workers and tenant fairness

Add multiple API/scheduler processes over shared durable state, weighted tenant admission, worker
capabilities, priority aging, retry budgets, and graceful drain. Migrate from the first scheduler
policy without losing jobs.

### Principal — queue platform evolution

Derive possible tiered service objectives from explicit user journeys, capacity allocation, regional failure behavior, cost per million jobs,
ownership boundaries, protocol compatibility, rollout verification, and rollback criteria.

## Failure and acceptance experiments

- Kill after publish persistence, after claim, after external effect, and before acknowledgement.
- Pause workers until queues saturate and verify bounded memory plus prompt admission rejection.
- Reuse stale lease tokens and prove they cannot mutate current delivery state.
- Run a noisy tenant beside small tenants and measure the configured fairness bound.
- Corrupt/truncate the final local record and demonstrate documented recovery.
- Drain and upgrade workers with two protocol versions while accepted work reaches a defined state.

Ship dashboards and alerts for admission errors, depth, oldest age, lease expiry, retries, dead
letters, storage latency, and worker saturation. Provide runbooks for backlog, poison jobs, disk
pressure, clock error, and rollback. Review the result with the
[evaluation rubric](../../../research/system-design-curriculum/evaluation-rubric.md).

## Operational readiness

Define readiness separately for admission, scheduling, storage, and workers. Alert on sustained age,
rejection, lease expiry, dead letters, disk pressure, and fairness breach; exercise deploy, drain,
backup/restore, and scheduler rollback locally.

## Completion evidence and review

Submit the Rust services/clients, protocol and storage version notes, workload generator, benchmark
report, failure matrix, threat model, ADRs, dashboards, alerts, and runbooks. Authenticate producers,
workers, and operators; authorize queues by tenant; redact payloads by default; and bound payload,
history, and inspection APIs. Defend the delivery guarantee, fairness bound, saturation policy,
cost driver, hardest migration, and the guarantee intentionally not provided.

## Architecture review questions

- Why is the chosen storage and lease model sufficient for the acknowledgement contract?
- Where does backpressure appear, and what is sacrificed first at saturation?
- How will scheduler and protocol evolution avoid lost or double-owned work?
