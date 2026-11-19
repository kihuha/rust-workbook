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

## Stretch goals

Add parallel chunk requests, bandwidth limits, compression, or encrypted transport.
