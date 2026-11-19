# 04 — Databases: ShardStore distributed key-value service

## Business scenario

A feature-configuration platform outgrows one machine. Reads must remain fast, acknowledged writes
must survive a node loss, and adding capacity must not create an uncontrolled data-availability
event. Some stale reads are acceptable; lost confirmed configuration changes are not.

Build `shardstore`, a small Rust cluster with a versioned client protocol, partition ownership,
replication, recovery, and an operator CLI. Reuse LogStore, BranchStore, or another documented local
engine instead of hiding local durability inside the distributed lesson.

## Problem statement

Distribute a key space across replaceable nodes without losing the advertised read/write guarantees
during node failure, repair, hot-key load, or ownership movement.

## Learning objectives

- partition and replicate state under an explicit consistency contract;
- implement repair, rebuild, rebalance, and overload boundaries;
- distinguish local durability from distributed acknowledgement;
- test histories and measure skew, amplification, and recovery cost.

## Prerequisites and research

Complete [LogStore](../03-logstore/README.md) or [BranchStore](../02-branchstore/README.md). Study
[Brown Rust Book Chapters 16–17](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), the
[Dynamo paper](https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf), and the
[DDIA resources](https://dataintensive.net/).

## System boundaries and constraints

Use a small fixed-size local cluster and bounded key/value API. SQL, secondary indexes, transactions
across keys, production membership services, and transparent multi-region operation are out of scope.

## Required behavior

- Get, put, conditional put, and delete bounded keys/values.
- Partition by a justified range or hash scheme and expose ownership metadata.
- Replicate each partition and use a stated read/write acknowledgement policy.
- Version values, detect conflicts, and resolve or surface them according to the contract.
- Transfer ownership safely when nodes join/leave; throttle rebalance work.
- Repair missing/stale replicas and recover a replacement node from a snapshot plus changes.
- Protect the cluster from hot keys, slow replicas, and unbounded hinted data.

## Correctness and workload

Quorum-like counts do not automatically imply linearizability. Specify which operations are
linearizable, sequential, read-your-writes, monotonic, or eventually consistent, then test that
contract. An acknowledged durable write must meet the stated node-failure guarantee.

Use at least 1 million keys, 80% reads, 15% writes, 5% deletes, Zipf-like skew, values from 100 B
to 32 KiB, and bursts during rebalance. Report throughput, p50/p95/p99, stale/conflict rate,
replication/rebuild lag, bytes moved, disk amplification, and hot-partition saturation.

## Milestones

### Senior — static partitioned cluster

Implement routing, replication, versioned writes, consistency tests, node restart, and telemetry for
a fixed topology.

### Staff — repair and safe rebalance

Add membership changes, streaming transfer, read repair or anti-entropy, bounded hints, hot-key
protection, overload behavior, and rolling protocol/storage compatibility.

### Principal — data-platform evolution

Define multi-region placement, tenant isolation, residency, RPO/RTO, cost per GiB/operation,
ownership, deprecation, and migration from one partitioning or consistency policy to another.

## Failure and acceptance experiments

- Crash coordinator, primary/replicas, and clients at every acknowledgement boundary.
- Partition replicas and verify allowed reads/writes plus convergence after healing.
- Add/remove a node under load without lost acknowledged data or double ownership.
- Make one key and one partition hot and demonstrate bounded degradation.
- Fill a disk, slow one replica, corrupt a transferred chunk, and rebuild a node.
- Run a history checker appropriate to the advertised consistency level.

Ship cluster/partition metrics and runbooks for unavailable quorum, rebalance stall, replica
divergence, disk pressure, hot partitions, failed upgrade, and restore. Document what the fault
harness cannot prove.

## Operational readiness

Expose ownership epochs, replica health/lag, repair/rebuild/rebalance work, hot keys/partitions,
storage errors, and admission. Exercise node replacement, snapshot restore, disk pressure, rolling
protocol/storage upgrade, rebalance cancellation, and rollback.

## Completion evidence and review

Submit the cluster, client and operator tools, consistency/history tests, workload and fault
harnesses, benchmark/amplification report, threat model, ADRs, compatibility plan, and runbooks.
Authenticate nodes and clients, authorize key prefixes or tenants, encrypt transport, and bound all
repair/scan/admin work. Defend the exact consistency claim, rebalance safety, failure detector,
hot-key policy, replica cost, and migration from the original partition map.

## Architecture review questions

- What exact history is allowed by each read/write mode?
- How are stale owners fenced during overlapping transfer and client retry?
- Which bottleneck dominates rebuild, and how is foreground traffic protected?
