# Database projects

Production-style storage engines and data services with durable formats, indexes, queries, compaction, recovery, compatibility, and performance constraints.

| Project | Company scenario | Core delivery |
| --- | --- | --- |
| [01 — PulseStore](01-pulsestore/README.md) | Industrial-monitoring company | Embedded time-series storage engine |
| [02 — BranchStore](02-branchstore/README.md) | Inventory platform | Page-oriented B+ tree key-value engine |
| [03 — LogStore](03-logstore/README.md) | Offline-first field-service platform | Log-structured merge-tree key-value engine |
| [04 — ShardStore](04-shardstore/README.md) | Global feature-configuration platform | Partitioned and replicated key-value service |

Add future database briefs as sibling project directories with their own `README.md`.

## DDIA reading path for storage projects

Both projects turn the OLTP storage-engine ideas in Martin Kleppmann and Chris Riccomini's *Designing Data-Intensive Applications*, 2nd Edition (O'Reilly, 2026) into small Rust systems. Read [Chapter 4, “Storage and Retrieval”](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/ch04.html) in this order:

1. **Storage and Indexing for OLTP** for the workload and index vocabulary.
2. **Log-Structured Storage** before starting LogStore.
3. **B-Trees** before starting BranchStore.
4. **Comparing B-Trees and LSM-Trees** before running the final cross-project benchmark.

The projects deliberately use a shared key-value API so that the storage strategy, rather than the application interface, explains the differences you measure. The chapter is the conceptual reference, not a complete implementation specification: record the extra decisions you make about page layout, crash consistency, compaction, concurrency, and operating-system durability.

Use the [official second-edition book page](https://www.oreilly.com/library/view/designing-data-intensive-applications/9781098119058/) to verify the edition and table of contents. Section names are given instead of page numbers because print and digital pagination can differ.

The companion [DDIA 2nd-edition storage-engine research note](../../research/ddia2-storage-engines.md) maps these sections to the project requirements and links the authors' official Chapter 4 bibliography, the original B-tree and LSM-tree papers, and production examples in SQLite and RocksDB.

Complete BranchStore or LogStore before ShardStore. The first two expose local durability and
indexing; ShardStore makes replication, partitioning, failure detection, and consistency visible.
