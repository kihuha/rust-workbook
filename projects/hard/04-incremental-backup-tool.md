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

## Stretch goals

Add compression, encryption, retention policies, or garbage collection for unreferenced blobs.
