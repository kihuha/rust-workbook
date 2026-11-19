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

## Stretch goals

Stream results for very large files or compare channel and Rayon implementations.
