# DDIA 2nd edition: B-tree and log-structured storage research

Research date: 2026-11-19

## Scope and source confidence

This note supports two Rust storage-engine project briefs. It focuses on the two OLTP storage families in *Designing Data-Intensive Applications*, 2nd edition (DDIA2), rather than attempting to reproduce the book.

The publisher identifies the book as Martin Kleppmann and Chris Riccomini's February 2026 second edition (ISBN 9781098119058). Its Chapter 4, **Storage and Retrieval**, explicitly contains **Storage and Indexing for OLTP**, **Log-Structured Storage**, **B-Trees**, and **Comparing B-Trees and LSM-Trees**. The publisher's public preview also states the chapter's purpose: understanding storage internals helps an engineer select and tune an engine for a workload. These chapter and section names are directly verified against O'Reilly's catalog and chapter preview. ([O'Reilly book page](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/), [O'Reilly Chapter 4 preview](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/ch04.html))

Page numbers are intentionally omitted below. O'Reilly's public preview verifies the headings but does not expose stable print pagination, and ebook pagination can vary. A learner should use **Chapter 4 + section title** as the durable locator in their own print or ebook copy.

The authors maintain an official companion repository of further-reading links. It identifies itself as accompanying DDIA2, is copyright 2026 Martin Kleppmann and Chris Riccomini, and has a dedicated Chapter 4 bibliography. ([official DDIA2 references repository](https://github.com/ept/ddia2-references), [Chapter 4 references](https://github.com/ept/ddia2-references/blob/main/chapter-04-refs.md))

## Reading map for learners

Read these parts of DDIA2 Chapter 4 in order:

1. **Storage and Indexing for OLTP** — establish why an index improves reads while adding write and space costs.
2. **Log-Structured Storage** — follow the progression from append-only records to sorted immutable files and background merging.
3. **B-Trees** — study page-oriented, update-in-place ordered indexes and the machinery needed to survive page mutation.
4. **Comparing B-Trees and LSM-Trees** — compare workload-dependent read, write, and space amplification rather than declaring one universally superior.
5. Optionally read **Multicolumn and Secondary Indexes** and **Storing Values Within the Index** after the core engine works; they are extension topics, not MVP requirements. These are all verified Chapter 4 section titles in the publisher's contents. ([O'Reilly Chapter 4 preview](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/ch04.html))

For deeper reading, the authors' Chapter 4 bibliography points to the original LSM-tree paper, the original B-tree work, ARIES-style write-ahead recovery, Bloom's paper, an LSM survey, and the RUM conjecture. ([official Chapter 4 references, entries 10, 13, 15, 21, 24, and 27](https://github.com/ept/ddia2-references/blob/main/chapter-04-refs.md))

## Log-structured / LSM storage synthesis

The project-relevant model is:

- Treat the durable write path as sequential append. A simple append-only key-value log makes updates new records rather than overwrites; deletion is represented by a tombstone. This is the conceptual starting point for the chapter's log-structured family. The original LSM paper is the foundational source linked by the authors. ([O'Neil et al., *The Log-Structured Merge-Tree*](https://www.cs.umb.edu/~poneil/lsmtree.pdf), [official Chapter 4 bibliography](https://github.com/ept/ddia2-references/blob/main/chapter-04-refs.md))
- Buffer recent writes in an ordered in-memory structure (a memtable), protect those unflushed writes with a write-ahead log, and flush a full immutable memtable as a sorted string table (SSTable). RocksDB's project documentation confirms this concrete design: writes enter a memtable, reads check it before older SST files, and a full memtable becomes immutable and is flushed in the background. ([RocksDB MemTable documentation](https://github.com/meta/rocksdb/wiki/MemTable), [RocksDB WAL documentation](https://github.com/meta/rocksdb/wiki/Write-Ahead-Log-%28WAL%29))
- Reads reconcile multiple generations, with newer entries and tombstones shadowing older values. Per-file key ranges, sparse indexes, and optional Bloom filters avoid unnecessary SSTable reads; a Bloom filter can rule out a file but cannot prove that a key is present. RocksDB documents both its mutable/immutable/SST lookup path and per-SST Bloom-filter role. ([RocksDB SST indexing](https://github.com/meta/rocksdb/wiki/Indexing-SST-Files-for-Better-Lookup-Performance), [RocksDB Bloom filters](https://github.com/meta/rocksdb/wiki/RocksDB-Bloom-Filter))
- Background compaction merge-sorts files, discards shadowed versions when safe, carries tombstones until they can safely suppress all older values, and produces new immutable files before atomically publishing metadata. RocksDB describes compaction as merging files into larger files/levels and removing overwritten or deleted bindings; it also documents leveled, universal/tiered, and FIFO strategies. ([RocksDB overview](https://github.com/meta/rocksdb/wiki/RocksDB-Overview), [RocksDB compaction](https://github.com/meta/rocksdb/wiki/Compaction))
- Compaction is not mere cleanup. It controls read, write, and space amplification and can become the throughput bottleneck. RocksDB explicitly frames its LSM design as a flexible trade-off among those three amplification factors and says sustained write throughput depends on compaction speed. ([RocksDB README](https://github.com/meta/rocksdb), [RocksDB overview](https://github.com/meta/rocksdb/wiki/RocksDB-Overview))

### What the Rust LSM project should force the learner to build

- A binary, length-delimited, checksummed WAL with a documented `fsync` policy and safe truncated-tail recovery.
- An ordered memtable with a strict memory limit and an immutable handoff during flush.
- Versioned SSTables containing sorted entries, block or record checksums, min/max keys, and a sparse index.
- Point lookup and range scan that merge the memtable and multiple SSTables with newest-value/tombstone precedence.
- A manifest or version set published atomically so recovery never mistakes half-written output for live data.
- At least one compaction policy. Size-tiered compaction is the smallest coherent first implementation; leveled compaction is a valuable extension.
- Crash injection around WAL append/sync, SST creation/sync, rename, manifest publication, and obsolete-file deletion.
- Measurements of write, read, and space amplification plus compaction debt/backpressure.

This scope is a curriculum synthesis from the verified DDIA2 sections and primary implementation sources; it is not a claim that DDIA2 specifies this exact Rust API or file format.

## B-tree storage synthesis

The project-relevant model is:

- A B-tree is an ordered, balanced, page-oriented index with a root, internal separator keys/child pointers, and leaf entries. High fan-out keeps the tree shallow and makes point and range access proportional to tree height. The original Bayer–McCreight work and a major B-tree survey are in the authors' Chapter 4 bibliography. ([official Chapter 4 references, entries 21 and 22](https://github.com/ept/ddia2-references/blob/main/chapter-04-refs.md))
- Disk is managed in fixed-size pages. Search follows child page IDs; insertion updates a leaf and splits an overfull page, propagating a separator upward and sometimes creating a new root. Deletion needs an explicit policy for underfull pages (redistribute/merge immediately, defer rebalancing, or initially support tombstones only). SQLite's official format is a production example in which the database is page-based and distinguishes interior and leaf B-tree pages. ([SQLite database file format](https://www.sqlite.org/fileformat2.html), [SQLite architecture](https://www.sqlite.org/arch.html))
- Updating pages in place makes crash consistency a first-class problem. The project therefore needs a write-ahead log or a copy-on-write page strategy; a WAL must make the intended change durable before the corresponding data page. DDIA2's own Chapter 4 bibliography includes ARIES/IM specifically for concurrent index management with write-ahead logging. ([official Chapter 4 references, entry 24](https://github.com/ept/ddia2-references/blob/main/chapter-04-refs.md))
- A page cache separates logical B-tree operations from physical I/O. Dirty-page eviction, WAL ordering, checksums, page allocation/free lists, and root metadata are part of the storage engine, not incidental details. SQLite's architecture likewise places a page-cache/pager layer beneath its B-tree implementation. ([SQLite architecture](https://www.sqlite.org/arch.html))
- B-tree and LSM performance is workload- and hardware-dependent. The useful comparison is the mechanism: scattered B-tree updates may mutate several pages and generate WAL traffic, while an LSM batches writes into sequential files but pays later through lookup across runs and compaction. The book deliberately gives the comparison its own section, and its official bibliography links the RUM conjecture and amplification comparisons. ([O'Reilly Chapter 4 preview](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/ch04.html), [official Chapter 4 references, entries 27 and 40](https://github.com/ept/ddia2-references/blob/main/chapter-04-refs.md))

### What the Rust B-tree project should force the learner to build

- A versioned fixed-size page format with header, page type, page ID, entry count, checksum, and sibling/child references.
- A pager abstraction for allocate/read/write/free plus a bounded page cache with explicit dirty tracking.
- Point lookup, ordered range scan, insertion, leaf/internal splitting, root replacement, and duplicate-key update semantics.
- A WAL with transaction/LSN identifiers, redo-capable recovery, WAL-before-data enforcement, and idempotent replay. Undo or multi-operation transactions can remain an extension if the MVP exposes atomic single-key writes only.
- A durable database header or dual superblocks so a root change cannot leave the file without a valid root.
- Structural validation: sorted keys, valid separator ranges, uniform leaf depth, no cycles, valid page references, and no allocated/free-page overlap.
- Crash injection at page split and root-publication boundaries, plus randomized model-based tests against `BTreeMap`.
- Metrics for page reads/writes, cache hit ratio, tree height, split rate, WAL bytes, and space utilization.

This scope is likewise a curriculum synthesis, not a purported DDIA2 implementation recipe.

## Comparison that the two briefs should teach

| Question | Page-oriented B-tree | Log-structured / LSM tree |
| --- | --- | --- |
| Foreground write | Mutate one or more existing pages, protected by WAL or copy-on-write | Append WAL, update memtable, later flush immutable sorted data |
| Point read | Follow one root-to-leaf path, usually helped by a page cache | Check newest in-memory/on-disk sources in precedence order, pruned by ranges/indexes/filters |
| Range read | Start at a leaf and walk ordered sibling pages | Merge sorted iterators from the memtable and overlapping SSTables |
| Space reclamation | Reuse freed space/pages; rebalance policy matters | Compaction drops shadowed versions and eventually tombstones |
| Main background work | Checkpoint/flush dirty pages; optional vacuum/rebalance | Flush and compaction are essential for bounded reads and space |
| Failure-sensitive transition | Page split/root change and WAL/data ordering | Flush/compaction output publication and obsolete-file deletion |
| Amplification lesson | Page and WAL writes, cache misses, fragmentation | Read/write/space amplification shaped by compaction policy |

The table is an implementation-oriented inference from the sources above. It should become a shared benchmark/report requirement: use identical generated workloads, state cache and durability settings, and explain results rather than expecting one engine to win every workload.

## Recommended project boundary

Both projects should be single-process embedded byte-key/byte-value databases, not SQL databases and not distributed systems. Require `put`, `get`, `delete`, and bounded ordered `scan`; persistence; restart recovery; versioned formats; corruption detection; and a workload report. Keep concurrency modest (one writer with snapshot or otherwise documented readers) so the learning target remains storage layout and recovery.

Avoid hiding the core behind `sled`, RocksDB, SQLite, or another database crate. Utility crates for checksums, serialization, temporary directories, property testing, and CLI parsing are appropriate, but learners should implement the pager/WAL/B-tree or WAL/memtable/SSTable/compaction path themselves.

## Primary and authoritative reference list

- Martin Kleppmann and Chris Riccomini, *Designing Data-Intensive Applications*, 2nd ed., **Chapter 4: Storage and Retrieval**, especially **Storage and Indexing for OLTP**, **Log-Structured Storage**, **B-Trees**, and **Comparing B-Trees and LSM-Trees**. ([publisher page](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/), [Chapter 4 preview](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/ch04.html))
- Kleppmann and Riccomini's [official DDIA2 Chapter 4 references](https://github.com/ept/ddia2-references/blob/main/chapter-04-refs.md).
- Patrick O'Neil, Edward Cheng, Dieter Gawlick, and Elizabeth O'Neil, [*The Log-Structured Merge-Tree (LSM-Tree)*](https://www.cs.umb.edu/~poneil/lsmtree.pdf), 1996.
- Rudolf Bayer and Edward McCreight, [*Organization and Maintenance of Large Ordered Indices*](https://doi.org/10.1145/1734663.1734671), 1970.
- Goetz Graefe, [*Modern B-Tree Techniques*](https://doi.org/10.1561/1900000028), 2011.
- Chen Luo and Michael Carey, [*LSM-Based Storage Techniques: A Survey*](https://arxiv.org/abs/1812.07527), 2019.
- Meta, [RocksDB overview](https://github.com/meta/rocksdb/wiki/RocksDB-Overview), [memtables](https://github.com/meta/rocksdb/wiki/MemTable), and [compaction](https://github.com/meta/rocksdb/wiki/Compaction), as a production LSM example.
- SQLite, [architecture](https://www.sqlite.org/arch.html) and [database file format](https://www.sqlite.org/fileformat2.html), as a production page/B-tree example.
