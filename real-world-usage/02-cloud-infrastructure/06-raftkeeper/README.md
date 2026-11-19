# 06 — Cloud infrastructure: RaftKeeper coordination service

## Business scenario

Infrastructure controllers need a small, strongly consistent store for configuration versions,
leader ownership, and fencing tokens. A stale controller performing work after losing leadership
could violate deployment safety.

Build `raftkeeper`, a three- or five-node Rust service implementing a narrow replicated state
machine. Do not use it for secrets or high-volume application data.

## Problem statement

Keep a small configuration and ownership state machine safe through node and network failures while
making the availability cost of quorum and strong reads observable.

## Learning objectives

- implement leader election, log replication, commit, recovery, and snapshots;
- distinguish safety from liveness and linearizable from stale reads;
- use terms, indexes, and fencing tokens to reject stale owners;
- test partitions, latency, restarts, membership changes, and unavailable majorities.

## Prerequisites and research

Complete [DurableQueue](../05-durablequeue/README.md) or the
[persistent KV store](../../../projects/hard/02-persistent-key-value-store.md). Study
[Brown Rust Book Chapters 15–17](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), the
[Raft paper and visualizations](https://raft.github.io/), and the
[distributed-systems guide](../../../research/system-design-reading-guide.md).

## System boundaries and constraints

Implement the Raft learning core rather than production TLS PKI, multi-Raft sharding, or a general
database. A deterministic in-process network must make message faults reproducible before sockets.

## Required behavior

- Replicate versioned `put`, conditional update, delete, and read operations.
- Elect at most one leader per term and redirect or reject writes at followers.
- Persist term, vote, log, snapshot metadata, and committed state across restart.
- Serve linearizable reads and optionally documented stale follower reads.
- Grant renewable leases whose monotonically increasing fencing token reaches protected resources.
- Add/remove one member at a time using a safe documented membership procedure.

## Workload and invariants

Model 90% reads and 10% writes, values from 64 B to 64 KiB, 50 concurrent clients, and periodic
1-second bursts. No two leaders may commit conflicting entries for the same log position; committed
entries survive a minority failure; applied state follows committed log order; stale lease holders
cannot pass the protected-resource fence.

Measure commit and read p50/p95/p99, election time, replication lag, snapshot pause, disk growth,
and availability while one node is slow. Run at increasing load until queueing dominates.

## Milestones

### Senior — replicated log

Build an in-process deterministic simulation first, then real processes with durable logs. Prove
election, replication, restart recovery, and state-machine agreement.

### Staff — snapshots, reads, leases, and membership

Add log compaction, snapshot installation, linearizable reads, fencing tokens, bounded request
queues, slow-follower protection, and a safe membership change.

### Principal — coordination as a platform

Define tenant limits, disaster recovery, upgrade compatibility, quorum placement, operator access,
capacity limits, ownership, and a migration from another coordination service without dual leaders.

## Failure and acceptance experiments

- Partition the old leader from a majority and prove it cannot commit or pass a newer fence.
- Randomly drop, duplicate, delay, and reorder messages in the deterministic simulation.
- Crash every persistence boundary and recover without two committed histories.
- Remove quorum and demonstrate unavailable writes rather than unsafe progress.
- Install a snapshot on a far-behind node, then rejoin it without state divergence.
- Perform a rolling binary/protocol upgrade and an aborted membership change.

Use property/state-machine tests plus a history checker for the advertised read/write contract.
Provide cluster-health metrics and runbooks for lost quorum, disk failure, membership mistakes,
snapshot failure, and rollback. Do not claim formal proof; identify what testing cannot establish.

## Operational readiness

Expose quorum, role/term, commit/apply lag, log/snapshot size, request queues, and election duration.
Alert on lost quorum, repeated elections, slow followers, disk pressure, and snapshot failure;
exercise rolling upgrade, backup/restore, membership repair, and rollback.

## Completion evidence and review

Submit the deterministic simulator, multi-process cluster, history/property tests, workload and
fault harnesses, benchmark report, protocol/storage compatibility notes, ADRs, and operator runbooks.
Use authenticated, authorized administrative operations and encrypted node transport. Defend the
linearizability boundary, quorum placement, stale-read policy, liveness limitations, recovery
assumptions, and why a simpler database lease would or would not meet the original requirement.

## Architecture review questions

- Which operations truly require linearizability, and what would a weaker contract save?
- What failure can leave the cluster safe but unavailable, and how is it diagnosed?
- How are stale leaders fenced outside RaftKeeper itself?
