# 08 — Cloud infrastructure: RegionGate global quota service

## Business scenario

A global API must enforce customer quotas with low-latency decisions in three regions. Requiring a
cross-ocean consensus round for every request is too slow, while fully independent counters can
overshoot a customer's purchased limit without bound.

Build `regiongate`: three local Rust decision services and a global allocation/reconciliation
control plane. The core exercise is choosing and measuring the correctness boundary.

## Problem statement

Make low-latency regional decisions while placing a quantified bound on global quota overshoot and
retaining an auditable usage total through partitions, retries, and failover.

## Learning objectives

- model the trade-off between local availability and global correctness;
- implement fenced quota grants and idempotent reconciliation;
- quantify overshoot, clock effects, failover time, and capacity reserve;
- migrate decision policy with shadow comparisons and rollback.

## Prerequisites and research

Complete [TenantPlane](../07-tenantplane/README.md) and preferably
[RaftKeeper](../06-raftkeeper/README.md). Review [Brown Rust Book Chapters 16–17](https://rust-book.cs.brown.edu/ch16-00-concurrency.html),
the [Dynamo paper](https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf), the
[Spanner paper](https://research.google/pubs/spanner-googles-globally-distributed-database/), and
the [SRE material](https://sre.google/books/).

## System boundaries and constraints

Regions are local processes behind a controllable network/clock harness. This is not a billing or
edge deployment product; global authentication, taxation, and real cloud routing remain simulated.

## Requirements and contract

- Make a local allow/deny decision when the region is healthy.
- Support per-second burst limits and monthly global usage budgets.
- Allocate, borrow, expire, and reconcile regional quota grants with version/fencing protection.
- Route tenants according to residency policy and expose regional health.
- Survive a region or inter-region link failure with an explicit degraded policy.
- Produce an auditable global usage and overshoot report.

Define the maximum allowed overshoot mathematically. State whether a stale region may admit work,
which products fail closed, and how clock uncertainty affects windows. No acknowledged usage may
disappear from billing reconciliation.

## Workload and decisions

Simulate three regions, 1,000 tenants, a 90/9/1 traffic skew, bursty retries, 100 ms inter-region
latency, and partitions lasting 1–10 minutes. Measure local p99 decision latency, denied-good and
allowed-over-budget decisions, overshoot, reconciliation lag, failover time, and bandwidth.
Compare escrow/allocation, centralized checks, CRDT-style counters, and fixed regional splits;
active-active with active-passive; and fail-open with fail-closed policy.

## Milestones

### Senior — local limiter with durable usage

Implement local token buckets, idempotent usage records, bounded queues, and replayable reporting.

### Staff — regional grants and reconciliation

Add grant allocation, versioning, borrowing, partition behavior, regional failover, residency
routing, and a verifier that reconstructs global totals.

### Principal — global operating model

Set product-specific correctness policies, RPO/RTO, capacity reserve, transfer costs, incident
authority, rollout stages, and a migration from a centralized limiter with shadow comparisons.

## Required experiments

- Partition every pair of regions and verify the documented overshoot bound.
- Duplicate and reorder usage reports without double counting.
- Fail a region at peak load and measure recovery plus remaining-region saturation.
- Skew wall clocks and prove the chosen window semantics.
- Shadow old and new limiters, compare decisions, then exercise rollback.
- Demonstrate a residency-constrained tenant never stores usage in a prohibited region.

Provide region/tenant dashboards, cost and capacity estimates, and runbooks for partition, quota
exhaustion, corrupt reconciliation, bad policy rollout, and regional evacuation.

## Operational readiness

Expose grant age/version, local admission, reconciliation lag, measured overshoot, clock offset,
regional capacity, and policy version. Exercise regional evacuation, reconciliation rebuild, bad
policy rollback, stale-grant revocation, and residency audit.

## Completion evidence and review

Submit three region processes, control plane, deterministic partition/clock harness, mathematical
overshoot argument, measurements, threat model, ADRs, migration plan, dashboards, and runbooks.
Authenticate inter-region reports and tenant policy changes, fence stale grants, and audit overrides.
Defend the consistency boundary, fail-open/closed choices, residency enforcement, capacity reserve,
RPO/RTO, and the business cost of tighter global correctness.

## Architecture review questions

- What is the mathematical overshoot bound during each partition scenario?
- Which products fail open or closed, and which user harm drives that decision?
- How can the allocation policy change without two authorities spending the same quota?
