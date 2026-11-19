# 02 — Databases: BranchStore B+ tree engine

## Company and service

StockRoom is a fictional inventory platform whose edge appliances repeatedly read and update a working set of product, bin, and reservation records. Operators also need ordered scans such as “all product keys from A through C.” The database must provide predictable reads without periodically compacting the entire dataset.

Build `branchstore`, an embedded ordered key-value database backed by a disk-resident B+ tree. Optimize for one writer, concurrent readers, point lookups, small updates, and bounded range scans.

## Read this first

Use *Designing Data-Intensive Applications*, 2nd Edition by Martin Kleppmann and Chris Riccomini as the conceptual starting point:

- [Chapter 4, “Storage and Retrieval”](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/ch04.html): read **Storage and Indexing for OLTP**, **B-Trees**, and **Comparing B-Trees and LSM-Trees**.
- Use **B-Trees** to understand fixed-size pages, branching, page splits, the write-ahead log used for crash recovery, and the reason implementations commonly add latches and copy-on-write techniques.
- Use **Comparing B-Trees and LSM-Trees** to form hypotheses about read amplification, write amplification, locality, fragmentation, and predictable performance. Test those hypotheses rather than treating them as universal results.

The publisher's [second-edition contents page](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/) identifies this material as Chapter 4. The book explains design families and trade-offs; this brief supplies a deliberately smaller implementation boundary.

Before coding, write a one-page design note answering:

1. Why is this a B+ tree rather than a textbook B-tree?
2. Which bytes make up an internal entry, leaf entry, and page header?
3. Which mutations must be atomic across a crash?
4. What is your policy for underfull pages after deletion?

## Brown Rust Book prerequisites

Study [ownership and borrowing](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [enums and pattern matching](https://rust-book.cs.brown.edu/ch06-00-enums.html), [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [errors](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-00-generics.html), [testing](https://rust-book.cs.brown.edu/ch11-00-testing.html), [smart pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), and [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html).

## Product boundary

Store arbitrary byte-string keys and values. Keys are ordered lexicographically by their encoded bytes. Set conservative configured limits, such as 1 KiB keys and 64 KiB values, and reject records that cannot fit your chosen representation.

Required operations:

```rust
let db = BranchStore::open(path, Options::default())?;
db.put(b"part/0042", b"17")?;
let value = db.get(b"part/0042")?;
let rows = db.range(b"part/0000"..b"part/1000")?;
db.delete(b"part/0042")?;
db.flush()?;
```

Define whether range bounds are inclusive or exclusive. Return a streaming iterator or cursor so a caller cannot accidentally load an unbounded scan into memory.

Do not implement SQL, distributed replication, secondary indexes, or a general transaction language in the required scope.

## Required storage design

### Page file and B+ tree

- Use a versioned database file divided into fixed-size pages, with a configurable default such as 4 KiB.
- Give every page a header containing its type, page ID, generation or log position, used-space metadata, and checksum.
- Keep separator keys and child page IDs in internal pages. Keep complete key/value records in linked leaf pages.
- Search from the current root to a leaf. Support binary search within a decoded page or an equivalent ordered slot directory.
- Split overflowing leaf and internal pages, propagate separator keys upward, and create a new root when required.
- Maintain sibling links between leaves for ordered scans.
- Reuse pages only through a durable free-page structure. A leaked page is preferable to reusing a page that may still be reachable after recovery.

### Deletion policy

Implement one documented policy:

- **Required baseline:** delete from leaves, preserve correct search and scan behavior, and permit underfull pages.
- **Stretch policy:** borrow from siblings and merge underfull pages, including root contraction.

Whichever policy you choose, bound or measure space fragmentation and provide a verification command that detects unreachable or multiply referenced pages.

### Durability and recovery

- Write a checksummed WAL record before flushing any page mutation it describes.
- Assign monotonic log sequence numbers and make page updates idempotent during recovery.
- Define the commit point for `put` and `delete`, including when WAL bytes are synchronized.
- Use a crash-safe metadata scheme, such as alternating meta pages, so recovery can identify a valid root and checkpoint.
- On startup, reject incompatible formats, select the latest valid metadata, replay committed WAL records, ignore a truncated final record, and report corruption elsewhere.
- Checkpoint the WAL only after all covered pages and metadata are durable in the documented order.

Do not claim durability without documenting assumptions about `fsync`, directory synchronization, sector writes, and the supported filesystems.

## Suggested Rust architecture

```text
src/
  lib.rs          public database API and error types
  codec.rs        endian-stable integers and record framing
  page.rs         page headers, slots, checksums, encode/decode
  pager.rs        page reads, cache, dirty-page tracking, free list
  tree.rs         search, insert, split, delete, and range cursor
  wal.rs          records, synchronization, replay, checkpoint
  recovery.rs     meta-page selection and startup recovery
  verify.rs       structural and reachability checks
```

Keep serialized page representations separate from mutable Rust structs. Never persist `usize`, native-endian integers, enum discriminants, or struct memory layouts directly.

## Tree invariants

The verifier and tests must establish that:

- keys within every page are strictly ordered;
- internal separators route every key to the correct child;
- all reachable leaves occur at the same depth;
- every non-root page has exactly one parent;
- leaf sibling links produce the same ordered sequence as a tree traversal;
- allocated, free, and unreachable page sets do not overlap;
- checksums and page IDs match the requested pages;
- reopening after any acknowledged operation produces the same visible mapping.

## Required tests

### Unit and property tests

- `T01` — Page and WAL codecs round-trip generated records without relying on host endianness.
- `T02` — Leaf insertion preserves ordering at the first, middle, and last position.
- `T03` — Generated insertion orders trigger leaf, internal, and root splits while preserving all invariants.
- `T04` — Point lookup and range scan match a `BTreeMap` model across generated operation sequences.
- `T05` — Deletion follows the documented underflow policy and never misroutes a remaining key.
- `T06` — Corrupted page and WAL checksums are detected; only a truncated WAL tail is safely ignored.

### Integration and crash tests

- `T07` — Acknowledged writes and deletes survive close and reopen.
- `T08` — Kill the process at every WAL append, sync, page write, meta-page swap, and checkpoint boundary; recovery yields either the pre-operation or committed state.
- `T09` — Concurrent readers observe the documented snapshot or locking semantics during splits.
- `T10` — The verifier detects injected cycles, bad separators, duplicate ownership, and free-list corruption.
- `T11` — A database created by the previous supported format version remains readable or fails with an explicit upgrade error.

### Performance tests

- `T12` — Measure random point reads, ascending scans, and random updates at several cache sizes.
- `T13` — Report tree height, page occupancy, cache hit rate, bytes written per user byte, and WAL size.
- `T14` — Repeat a mixed workload long enough to expose fragmentation, file growth, and page-cache leaks.

Performance results must include dataset size, key/value distribution, cache state, storage medium, sync mode, page size, and compiler profile.

## Observability and operations

Expose operation counts and latency, page reads and writes, cache hit rate, tree height, page splits, page occupancy, WAL append and sync latency, checkpoint duration, recovery actions, file size, and free-page count.

Ship `branchstore verify`, `branchstore dump-page <id>`, and `branchstore stats` commands. Provide backup instructions that either stop writes or use a consistent snapshot. Include runbooks for checksum failure, WAL corruption, an interrupted checkpoint, disk-full errors, and a failed format upgrade.

## Security and resource limits

Treat database files as untrusted input. Bounds-check offsets and lengths before allocation or slicing; reject impossible page types, counts, and child IDs; avoid recursive traversal without a depth limit; and fuzz every decoder. Never log raw keys or values by default.

## Acceptance criteria

BranchStore is complete when the model-based tests, invariant checker, crash matrix, compatibility test, and documented benchmarks pass against a versioned format, and when the results explain at least two measured consequences of choosing a B+ tree.

## Optional extensions

Add prefix-compressed keys, overflow pages for large values, a clock or LRU page cache, copy-on-write checkpoints, group commit, reader snapshots, or online file compaction.
