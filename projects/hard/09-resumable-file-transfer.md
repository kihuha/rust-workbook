# Hard 09 — Resumable file transfer

## Project challenge

Build a client and server that transfer large files in verified chunks and resume after interruption. The challenge is to define a robust protocol and never mistake partial data for a complete file.

## Required behavior

- Negotiate file name, total size, chunk size, and whole-file hash.
- Write chunks to a temporary destination with a persisted completion map.
- Resume by requesting only missing chunks.
- Verify each chunk and the completed file.
- Enforce destination path safety and configurable size limits.
- Atomically publish the destination only after full verification.

## Example resume report

```text
file: archive.bin
size: 104857600 bytes
chunks complete: 72/100
chunks requested: 28
final hash: verified
```

## Architecture expectations

Separate framing, protocol messages, transfer state, storage, hashing, and connection lifecycle. Simulate dropped connections and corrupted chunks in integration tests.

## Rust skills combined

Async networking, protocol framing, random-access file I/O, hashing, persisted state, path validation, retries, atomic publishing, and fault-injection tests.

## Done when

Interrupted transfers resume without retransmitting valid chunks, corrupted data is rejected, path traversal is impossible, and incomplete files never appear under the final name.

## Brown Rust Book prerequisites

Study [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [file I/O](https://rust-book.cs.brown.edu/ch12-02-reading-a-file.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), and [async tasks](https://rust-book.cs.brown.edu/ch17-00-async-await.html).

## Suggested tests

### Unit tests

- `T01` — Encode and decode every protocol frame.
- `T02` — Reject unsafe destination names and oversized metadata.
- `T03` — Detect a corrupted chunk before marking it complete.
- `T04` — Resume planning requests only missing chunks.

### Integration tests

- `T05` — Interrupt at 72 of 100 chunks and retransmit only 28.
- `T06` — Final bytes and whole-file hash match the source.
- `T07` — Incomplete or corrupt data never appears under the final name.

## Stretch goals

Add parallel chunk requests, bandwidth limits, compression, or encrypted transport.
