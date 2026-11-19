# 01 — Databases: PulseStore time-series engine

## Company and service

PulseForge is a fictional industrial-monitoring company. Its edge gateways collect high-frequency measurements and need an embedded database that keeps accepting data during network outages.

Build `pulsestore`, an embeddable time-series engine plus an optional local HTTP service. Optimize for one writer, concurrent readers, bounded disk use, and crash recovery.

## Brown Rust Book prerequisites

Study [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [errors](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits and lifetimes](https://rust-book.cs.brown.edu/ch10-00-generics.html), [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html), [smart pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), and [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html).

## Data model

```text
SeriesKey = metric name + sorted string labels
Point = timestamp_ns:i64 + value:f64
```

Example series: `temperature_c{device="pump-7",zone="north"}`.

Reject invalid metric names, duplicate label names, non-finite values unless explicitly supported, and timestamps outside the configured retention window.

## Required storage design

- Append accepted writes to a checksummed write-ahead log before acknowledgment.
- Buffer recent points in a bounded in-memory table.
- Flush immutable sorted segments with a versioned file format.
- Maintain an index from series keys and time ranges to segments.
- Compact segments without blocking reads or losing concurrent writes.
- Delete data only after retention and snapshot references permit it.
- Detect incomplete WAL and segment records during recovery.

## Library API

```rust
store.write_batch(points)?;
let rows = store.query(&series, start_ns..end_ns, QueryOrder::Ascending)?;
store.compact()?;
store.create_snapshot(path)?;
```

The query result should stream or iterate without loading an unbounded range into memory.

## Optional HTTP API

| Method | Endpoint | Purpose |
| --- | --- | --- |
| `POST` | `/v1/write` | Ingest a bounded batch of series points |
| `POST` | `/v1/query` | Query ranges and optional aggregation |
| `GET` | `/v1/series` | Find series by metric and label selectors |
| `POST` | `/v1/admin/compact` | Request compaction |
| `POST` | `/v1/admin/snapshot` | Create a consistent snapshot |
| `GET` | `/metrics` | Engine and process metrics |

## Example write and query

```json
{"series":{"metric":"temperature_c","labels":{"device":"pump-7"}},"points":[[1732000000000000000,24.1],[1732000001000000000,24.3]]}
```

```text
query pump-7 from t0 through t1 ascending
-> (t0, 24.1), (t1, 24.3)
```

## Correctness requirements

- Acknowledged points survive restart.
- Batch writes are all accepted or all rejected.
- Query ordering and time-bound semantics are documented.
- Duplicate timestamp policy is explicit.
- Readers see immutable snapshots or another documented consistency model.
- Compaction output is equivalent to its input according to that model.

## Required tests

### Unit and property tests

- `T01` — Series-key encoding is deterministic regardless of input label order.
- `T02` — WAL records detect bit corruption and truncated tails.
- `T03` — Segment encode/decode round-trips generated points.
- `T04` — Query merging returns sorted points across overlapping segments.
- `T05` — Compaction preserves visible query results.

### Integration tests

- `T06` — Acknowledged writes survive process restart.
- `T07` — Crash at every flush and manifest checkpoint recovers to a valid state.
- `T08` — Concurrent readers remain correct during flush and compaction.
- `T09` — Snapshot restore reproduces series, points, and metadata.
- `T10` — Retention removes only eligible data.

### Performance and longevity tests

- `T11` — Sustain the target write rate while keeping p99 ingest latency within budget.
- `T12` — Large-range queries remain within the memory limit.
- `T13` — Run repeated compaction for 24 hours without file leaks or index divergence.
- `T14` — Fuzz WAL, segment, manifest, and HTTP decoders.

## Observability

Report ingest count and latency, WAL sync duration, memory-table size, segment count, compaction backlog, disk use, query latency, cache hit rate, rejected points, and recovery actions.

Log file IDs and series fingerprints, not sensitive label values. Emit a clear recovery report after startup.

## Deployment and compatibility

Version every on-disk structure. Support read-before-upgrade compatibility checks, atomic manifest replacement, backup verification, and a documented downgrade boundary.

Ship as a Rust library and non-root sidecar image. Define filesystem durability assumptions and test on each supported filesystem and architecture.

## Service-level objectives

- No loss of acknowledged writes under the documented fsync mode.
- Recovery of a target-size database within the agreed startup window.
- Bounded memory during ingestion, querying, and compaction.
- On-disk compatibility across supported minor releases.

## Acceptance criteria

PulseStore is complete when crash injection, snapshot restore, compaction equivalence, concurrent reads, retention, and sustained-load tests pass against a versioned storage format.

## Optional extensions

Add compression, downsampling, label indexes, replication, remote object-storage snapshots, or SQL-like query syntax.
