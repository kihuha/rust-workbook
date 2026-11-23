---
schema: 1
kind: chapter
chapter: 16
slug: fearless-concurrency
title: "Fearless Concurrency"
---

# Chapter 16 — Fearless Concurrency

## Book alignment

Study [Fearless Concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html): threads, message passing, shared-state concurrency, and the `Send` and `Sync` traits.

## Mastery goals

- Move owned data safely into threads and join them.
- Coordinate producers and consumers with channels.
- Protect shared mutable state with `Arc<Mutex<T>>`.
- Design shutdown paths and avoid common deadlock patterns.

<!-- challenge
id: "16.1"
title: "Parallel checksum workers"
status: "not-started"
-->

## Challenge 16.1 — Parallel checksum workers

### Brief

Calculate checksums for many byte buffers using a fixed number of worker threads and a result channel.

### Required behavior

- Assign each input a stable job ID.
- Use at most the configured worker count.
- Send job results to the coordinator.
- Preserve errors per job instead of dropping all results.
- Join every worker before returning.

### Expected interface and `main.rs` flow

```rust
fn checksum(bytes: &[u8]) -> Result<u64, ChecksumError>;
fn calculate_all(
    jobs: Vec<Job>,
    worker_count: usize,
) -> Result<Vec<JobResult>, WorkerPoolError>;
```

`main` builds the jobs, calls `calculate_all` once, sorts returned results by stable job ID if needed, and prints them. `calculate_all` owns thread creation, channel lifetime, and joins. Workers call `checksum`; they do not print.

### Example

```text
job 1: [1, 2, 3] -> checksum 6
job 2: [10, 20]  -> checksum 30
completed: 2
```

### Suggested tests

#### Unit tests

- `T01` — Verify checksum of an empty buffer is zero.
- `T02` — Verify checksum of `[1, 2, 3]` is `6` under the documented algorithm.
- `T03` — Verify overflow follows a documented wrapping or error policy.

#### Integration tests

- `T04` — Verify ten jobs each produce exactly one result.
- `T05` — Verify active workers never exceed the configured limit.
- `T06` — Verify the function returns only after workers terminate.

### Done when

The coordinator owns lifecycle decisions and worker completion cannot be lost silently.

<!-- /challenge -->

<!-- challenge
id: "16.2"
title: "Compare channels and shared state"
status: "not-started"
-->

## Challenge 16.2 — Compare channels and shared state

### Brief

Implement a parallel word count twice: first with per-worker maps sent through a channel, then with one shared `Arc<Mutex<HashMap<...>>>`.

### Required behavior

- Produce identical normalized counts in both versions.
- Keep lock scopes minimal in the shared-state version.
- Use multiple producers in the channel version.
- Document contention, ownership, and failure tradeoffs.

### Expected functions and `main.rs` flow

```rust
fn count_sequential(chunks: &[&str]) -> HashMap<String, usize>;
fn count_with_channels(chunks: Vec<String>) -> Result<HashMap<String, usize>, CountError>;
fn count_with_shared_state(chunks: Vec<String>) -> Result<HashMap<String, usize>, CountError>;
```

`main` gives equivalent owned chunk sets to both concurrent functions, compares each result with `count_sequential`, then prints one sorted report. Counting functions return maps and errors; they do not print from worker threads.

### Example

```text
chunks: ["rust is fast", "rust is safe"]
is: 2
rust: 2
fast: 1
safe: 1
```

### Suggested tests

#### Unit tests

- `T01` — Verify normalization and local counting for one chunk.
- `T02` — Verify merging two local maps sums overlapping keys.

#### Integration tests

- `T03` — Verify both concurrent implementations equal a sequential reference.
- `T04` — Verify empty chunks do not deadlock or create false counts.
- `T05` — Verify repeated runs produce the same final map.

### Done when

You can justify which coordination model better matches this problem rather than choosing based only on shorter code.

<!-- /challenge -->
