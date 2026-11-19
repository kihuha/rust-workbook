# Medium 02 — Parallel log analyzer

## Project challenge

Build a tool that processes many server log files concurrently and merges partial reports. The challenge is to divide work without introducing shared mutable aggregation bugs.

## Required behavior

- Parse timestamp, level, service, and message from each log line.
- Process files through a fixed-size worker pool.
- Count events by level and service.
- Report malformed lines with file name and line number.
- Print the most frequent error messages with deterministic tie ordering.

## Example input

```text
2026-11-01T10:00:00Z INFO api started
2026-11-01T10:01:00Z ERROR api database-unavailable
2026-11-01T10:02:00Z ERROR worker database-unavailable
```

## Expected summary

```text
INFO: 1
ERROR: 2
api: 2
worker: 1
top error: database-unavailable (2)
```

## Rust skills combined

Threads, channels, ownership transfer, worker coordination, maps, parsers, filesystem traversal, error context, and benchmark-oriented tests.

## Done when

Parallel output matches a single-threaded reference, every file is processed once, malformed lines are counted separately, and workers shut down cleanly.

## System-design foundation

Bound files and records in flight, preserve deterministic results independent of worker order, and
measure throughput plus memory from one to saturation workers. Continue with
[EventLog](../../real-world-usage/11-messaging-streaming/01-eventlog/README.md) or SignalWorks.

### Foundation evidence

- `F01` — Instrument files/records in flight and assert configured bounds while one worker is deliberately stalled.
- `F02` — Benchmark 1, 2, 4, and 8 workers on the same corpus; report throughput, peak memory, and the saturation point.

## Brown Rust Book prerequisites

Study [hash maps](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html), [contextual errors](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), [iterator aggregation](https://rust-book.cs.brown.edu/ch13-02-iterators.html), and [threads and channels](https://rust-book.cs.brown.edu/ch16-00-concurrency.html).

## Suggested tests

### Unit tests

- `T01` — Parse each example line into the expected fields.
- `T02` — Merge two partial maps by summing shared keys.
- `T03` — A malformed line reports its file and line number.

### Integration tests

- `T04` — Parallel and sequential analyzers return identical reports.
- `T05` — Worker count stays bounded and all workers terminate.
- `T06` — Equal-frequency output remains deterministic across runs.

## Stretch goals

Stream results for very large files or compare channel and Rayon implementations.
