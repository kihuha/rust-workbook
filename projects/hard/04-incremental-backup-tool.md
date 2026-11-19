# Hard 04 — Incremental backup tool

## Project challenge

Build a content-addressed backup tool that stores unchanged file contents only once. The challenge is to create consistent snapshots while files, metadata, and storage failures can disagree.

## Required behavior

- Walk a source tree without following symlink cycles.
- Hash regular-file contents and store blobs by hash.
- Write a snapshot manifest containing relative paths, hashes, sizes, and selected metadata.
- Reuse existing blobs across snapshots.
- Restore a snapshot to an empty destination without overwriting existing files by default.
- Verify blob hashes and report missing or corrupt data.

## Example summary

```text
snapshot 2026-11-19T08:00:00Z
files scanned: 120
new blobs: 8
reused blobs: 112
bytes stored: 48201
```

## Architecture expectations

Separate traversal, hashing, object storage, manifests, restoration, and verification. Write manifests through a temporary file followed by an atomic rename where the platform permits.

## Rust skills combined

Filesystem traversal, streaming hashes, content-addressed storage, serialization, path safety, atomic file operations, recovery, and integration tests.

## Done when

Two identical snapshots store no duplicate blobs, restore reproduces file contents, a failed snapshot never appears complete, and path traversal outside the destination is impossible.

## System-design foundation

Define snapshot atomicity, verify content rather than trusting metadata, and crash every publication
boundary. Measure deduplication ratio, scan throughput, and restore time. These techniques recur in
database snapshots, replica rebuilds, and regional migration.

### Foundation evidence

- `F01` — Inject failure before every blob/manifest publication and prove no incomplete snapshot is listable or restorable.
- `F02` — Benchmark first, unchanged, and 10%-changed snapshots plus full restore; report throughput, bytes stored, deduplication, and verification time.

## Brown Rust Book prerequisites

Study [ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [error propagation](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), [file I/O](https://rust-book.cs.brown.edu/ch12-02-reading-a-file.html), and [iterator processing](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — Identical bytes produce the same content hash.
- `T02` — Manifest paths reject absolute and parent-traversal components.
- `T03` — Verification detects a modified blob.

### Integration tests

- `T04` — Two identical snapshots create no duplicate blobs.
- `T05` — Restore reproduces every file byte in a temporary tree.
- `T06` — An interrupted snapshot never becomes a visible complete manifest.
- `T07` — Restore refuses to overwrite existing files by default.

## Stretch goals

Add compression, encryption, retention policies, or garbage collection for unreferenced blobs.
