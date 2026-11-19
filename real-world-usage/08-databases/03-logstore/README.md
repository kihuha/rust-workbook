# 03 — Databases: LogStore LSM engine

## Company and service

FieldKit is a fictional offline-first field-service platform. A technician's device records inspections, parts used, and synchronization state in bursts while connectivity is unreliable. Writes must remain sequential and fast, while point reads and ordered synchronization scans must continue across many immutable files.

Build `logstore`, an embedded ordered key-value database based on a log-structured merge tree (LSM-tree). Optimize for one writer, concurrent readers, write-heavy workloads, bounded memory, and background compaction.

## Read this first

Use *Designing Data-Intensive Applications*, 2nd Edition by Martin Kleppmann and Chris Riccomini as the conceptual starting point:

- [Chapter 4, “Storage and Retrieval”](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/ch04.html): read **Storage and Indexing for OLTP**, **Log-Structured Storage**, and **Comparing B-Trees and LSM-Trees**.
- Use **Log-Structured Storage** to understand append-only records, hash indexes, sorted-string tables (SSTables), memtables, merge compaction, sparse indexes, and the role of Bloom filters.
- Use **Comparing B-Trees and LSM-Trees** to form hypotheses about sequential writes, write and read amplification, compaction interference, disk-space usage, and workload sensitivity. Validate them with your own engine.

The publisher's [second-edition contents page](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/) identifies this material as Chapter 4. An append-only log alone is not yet an LSM-tree; this project adds sorted in-memory state, immutable sorted files, lookup metadata, and compaction.

Before coding, write a one-page design note answering:

1. When is a write acknowledged, and what survives a crash at that point?
2. How does a lookup find the newest version across memory and multiple files?
3. When may a tombstone be discarded without resurrecting an old value?
4. Which compaction policy will you implement, and what behavior do you expect from it?

## Brown Rust Book prerequisites

Study [ownership and borrowing](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [errors](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-00-generics.html), [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html), [smart pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), and [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html).

## Product boundary

Store arbitrary byte-string keys and values. Keys are ordered lexicographically by their encoded bytes. Set configured limits for key size, value size, write batches, memory-table size, open files, and background work.

Required operations intentionally match BranchStore:

```rust
let db = LogStore::open(path, Options::default())?;
db.put(b"job/0042", b"complete")?;
let value = db.get(b"job/0042")?;
let rows = db.range(b"job/0000"..b"job/1000")?;
db.delete(b"job/0042")?;
db.flush()?;
```

Define range-bound semantics. Implement scans as a lazy merge iterator with bounded memory. Do not implement SQL, distributed replication, secondary indexes, or a general transaction language in the required scope.

## Required storage design

### Write path

- Frame every write or delete in a checksummed write-ahead log before acknowledgment.
- Assign a monotonic sequence number to each mutation so newer values win deterministically.
- Apply mutations to a bounded ordered memtable after the WAL append succeeds.
- When the memtable reaches its limit, make it immutable, rotate the WAL, and continue writes in a new memtable.
- Flush the immutable memtable to a new SSTable using a temporary file, synchronize it, atomically publish it through the manifest, and only then retire its WAL.
- Apply explicit backpressure when immutable memtables or compaction debt exceed configured limits.

### SSTable format and reads

- Store entries in key order using versioned, checksummed blocks.
- Include the key range, sequence-number range, block offsets, and checksum information in a footer that can be located from the end of the file.
- Build a sparse block index. Add a Bloom filter after the baseline engine works and measure its false-positive rate.
- For `get`, search mutable and immutable memtables first, then candidate SSTables from newest to oldest according to the compaction layout.
- For `range`, merge sorted iterators and emit only the newest visible version of each key.
- Represent deletion as a tombstone carrying a sequence number; never turn it into an immediate physical delete.

### Manifest, recovery, and compaction

- Record the live SSTable set and compaction state in a versioned manifest changed through atomic, checksummed edits or snapshots.
- At startup, replay valid WAL records not represented by durable SSTables, ignore only a truncated final WAL record, remove or quarantine incomplete temporary files, and reject overlapping metadata that violates the chosen layout.
- Implement either size-tiered or leveled compaction. State the invariant, selection rule, output-file sizing, and write-stall policy for your choice.
- Merge input files in key order, keep the newest visible version, preserve tombstones that might still hide older data, and install outputs before deleting inputs.
- Defer physical file deletion until active readers can no longer reference the old version.

Do not claim durability without documenting assumptions about `fsync`, directory synchronization, atomic rename, and the supported filesystems.

## Suggested Rust architecture

```text
src/
  lib.rs          public database API, snapshots, and error types
  batch.rs        atomic write-batch representation
  wal.rs          append, synchronization, framing, and replay
  memtable.rs     ordered in-memory versions and byte accounting
  sstable.rs      blocks, footer, sparse index, and iterators
  manifest.rs     version edits and live-file snapshots
  merge.rs        newest-version and range merge iterators
  compact.rs      compaction selection, merge, and installation
  recovery.rs     startup reconciliation and orphan handling
  verify.rs       offline structural and semantic checks
```

Use owned, reference-counted immutable table metadata for reader snapshots. Keep on-disk encodings separate from Rust memory layouts, and never persist `usize`, native-endian integers, or enum discriminants directly.

## Storage invariants

The verifier and tests must establish that:

- WAL and SSTable records are well-framed, bounded, and checksummed;
- keys within every SSTable are ordered by key and then by descending sequence number;
- index separators route lookups to the correct data block;
- every manifest-live file exists and every installed file has valid metadata;
- the compaction layout obeys its documented overlap rules;
- a lookup or scan returns at most the newest visible version of a key;
- tombstones are retained whenever an older value could otherwise reappear;
- reopening after any acknowledged operation produces the same visible mapping.

## Required tests

### Unit and property tests

- `T01` — WAL, manifest, block, and footer codecs round-trip generated values independently of host endianness.
- `T02` — WAL recovery detects corruption but safely ignores a truncated final record.
- `T03` — SSTables preserve ordering and return the same point and range results as their sorted input.
- `T04` — Generated operation sequences match a `BTreeMap` model before and after flushes and compactions.
- `T05` — Merge iteration resolves duplicate keys, sequence numbers, and tombstones correctly.
- `T06` — Bloom-filter tests, when enabled, demonstrate no false negatives and report the observed false-positive rate.

### Integration and crash tests

- `T07` — Acknowledged writes and deletes survive close and reopen.
- `T08` — Kill the process at every WAL rotation, SSTable write, file sync, manifest publication, compaction installation, and input deletion boundary; recovery produces a valid committed state.
- `T09` — Readers holding an old version remain correct while flush and compaction install new files.
- `T10` — Repeated updates and deletes never resurrect an obsolete value after full compaction.
- `T11` — Disk-full and permission errors leave prior data readable and temporary files recoverable.
- `T12` — A database created by the previous supported format version remains readable or fails with an explicit upgrade error.

### Performance and longevity tests

- `T13` — Measure sequential and random writes, random point reads, and ascending scans with compaction both caught up and backlogged.
- `T14` — Report write amplification, read amplification, space amplification, WAL sync latency, flush throughput, compaction throughput, and write-stall time.
- `T15` — Repeat overwrite-heavy and delete-heavy workloads long enough to expose tombstone buildup, space leaks, starvation, and unbounded compaction debt.

Performance results must include dataset size, key/value distribution, cache state, storage medium, sync mode, memtable size, compaction policy, and compiler profile.

## Cross-project experiment

Run the same reproducible workload against LogStore and [BranchStore](../02-branchstore/README.md). Include at least a write-heavy workload, a point-read-heavy workload, and a range-scan workload.

Compare throughput and tail latency alongside write, read, and space amplification. Explain where your observations agree with or complicate the trade-offs in DDIA Chapter 4. Do not declare one design universally faster from a single benchmark.

## Observability and operations

Expose operation counts and latency, WAL bytes and sync latency, mutable and immutable memtable bytes, SSTable count by level or tier, block-index and Bloom-filter effectiveness, flush duration, compaction bytes read and written, compaction debt, write stalls, obsolete-file backlog, recovery actions, and disk use.

Ship `logstore verify`, `logstore dump-table <id>`, `logstore manifest`, and `logstore stats` commands. Provide backup instructions based on a consistent live-file snapshot. Include runbooks for checksum failure, WAL corruption, manifest corruption, disk-full errors, compaction stalls, orphan files, and a failed format upgrade.

## Security and resource limits

Treat database files as untrusted input. Bounds-check all lengths, counts, block handles, and allocations; limit decompression if added; cap recovery work; avoid recursion without a depth bound; and fuzz every decoder and merge iterator. Never log raw keys or values by default.

## Acceptance criteria

LogStore is complete when model-based tests, crash injection, snapshot-safe compaction, tombstone tests, compatibility checks, and documented benchmarks pass against a versioned format, and when the cross-project report explains at least two measured consequences of choosing an LSM design.

## System-design progression

Treat the required engine as the Senior milestone. In addition to existing benchmark disclosure,
record p50/p95/p99 across increasing load until write stalls, compaction debt, cache pressure, or
storage bandwidth becomes dominant.

For Staff practice, migrate compaction policy and on-disk format with old readers, expose a
snapshot-plus-change transfer boundary, and test disk-full, slow storage, checksum failure, and
unbounded tombstone/hot-key workloads. Compare against BranchStore with identical inputs.

For Principal review, design its use inside [ShardStore](../04-shardstore/README.md): identify which
durability and compatibility guarantees belong locally versus to replication, estimate storage and
operational cost, and plan a rolling migration without simultaneous compaction/rebuild overload.

## Optional extensions

Add compressed blocks, prefix-compressed keys, snapshots, group commit, a block cache, multiple compaction strategies, rate-limited compaction, value-log separation, or blob files for large values.
