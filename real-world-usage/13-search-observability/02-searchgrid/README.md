# 02 — Search and observability: SearchGrid document search

## Business scenario

A documentation platform must make new and corrected pages searchable quickly while serving
relevant results under uneven query traffic. Deletion and permission changes must not leave private
content discoverable through stale indexes.

Build `searchgrid`, a Rust ingestion/indexing service, query service, and reindex/migration CLI.

## Problem statement

Keep versioned, permission-filtered search results relevant and fresh while indexes are incrementally
updated, merged, sharded, rebuilt, or partially unavailable.

## Learning objectives

- build immutable inverted-index segments and atomic generations;
- evaluate relevance as well as ordinary functional correctness;
- enforce permissions across results, snippets, caches, and stale generations;
- operate sharded queries and verified zero-downtime reindexing.

## Prerequisites and research

Complete the [local note index](../../../projects/medium/10-local-note-index.md) and preferably
[LogStore](../../08-databases/03-logstore/README.md). Review [Brown Rust Book Chapters 8 and 13](https://rust-book.cs.brown.edu/ch08-00-common-collections.html),
[Apache Lucene documentation](https://lucene.apache.org/core/), and the
[research guide](../../../research/system-design-reading-guide.md).

## System boundaries and constraints

Begin with lexical document search. Crawling, OCR, user-behavior ranking, paid ranking, full vector
infrastructure, and production language analysis are out unless introduced as later experiments.

## Requirements and invariants

- Ingest versioned create/update/delete events with stable document IDs.
- Tokenize, normalize, and build immutable searchable segments plus metadata.
- Query terms, phrases, filters, pagination, and deterministic relevance scores.
- Apply document/tenant permissions before returning hits or snippets.
- Incrementally merge segments and atomically publish index generations.
- Reindex with a new schema/analyzer while serving the old generation.
- Shard and replicate indexes, return an explicit result for partial shard failure.

A stale update cannot overwrite a newer document version. A permission revocation or deletion must
meet a declared visibility deadline. Published generations are internally consistent and rollback
remains possible until verification completes.

## Workload and decisions

Use at least 1 million scaled documents, mixed sizes/languages, 95% queries, 5% updates, skewed
terms, autocomplete bursts, and permission changes. Measure indexing throughput/lag, query
p50/p95/p99, recall/relevance on a labeled set, bytes/document, merge cost, cache hit rate, and
shard imbalance. Compare term/range sharding, lexical/hybrid retrieval, offset/search-after
pagination, refresh frequency, cache placement, and exact/approximate document frequency.

## Milestones

### Senior — local incremental search engine

Implement segments, indexing, ranking, filters, permission checks, merges, crash-safe generation
publication, and a versioned relevance evaluation set.

### Staff — distributed query and reindex

Add shards/replicas, scatter-gather with deadlines, hot-shard protection, incremental ingestion,
dual-generation reindex, shadow queries, verification, and rollback.

### Principal — search platform evolution

Define tenant tiers, freshness/relevance/cost objectives, regional and residency behavior, schema
governance, team ownership, and migration toward hybrid/vector retrieval without weakening access.

## Required experiments

Crash every index publication boundary, replay duplicate/out-of-order updates, revoke access during
cache use, lose a shard during query, create a hot term, and migrate analyzers under load. Verify
freshness, no unauthorized result, bounded partial behavior, relevance comparison, and rollback.
Provide runbooks for lag, merge debt, bad relevance rollout, corrupt segment, and permission breach.

## Operational readiness

Expose ingestion lag, generation versions, segment/merge debt, shard health/imbalance, query
latency/partials, cache authorization, relevance comparisons, and deletion deadlines. Exercise
segment recovery, reindex abort, bad-ranking rollback, shard replacement, and permission incident.

## Completion evidence and review

Submit ingestion and query services, index/reindex tools, labeled evaluation set, workload and fault
harnesses, benchmark/relevance/cost report, threat model, ADRs, dashboards, and runbooks. Apply
authorization before hits, snippets, caches, and logs can reveal content. Defend ranking evidence,
freshness and deletion deadlines, shard choice, partial-result policy, reindex verification, storage
cost, and the boundary between lexical and later vector retrieval.

## Architecture review questions

- What evidence shows ranking improved rather than merely changed?
- How do deletion and permission revocation propagate through indexes and caches?
- How is an old index generation retained, verified, and removed safely?
