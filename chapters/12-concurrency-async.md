# Chapter 12 — Concurrency and async/await

## Challenge 034 — Parallel word counts

**Challenge:** Move independent text chunks to worker threads and merge their results without sharing one mutable map.

**Your task:** Spawn one worker per chunk. Each worker builds its own word-frequency map and sends it through a channel. The main thread merges all maps and joins every worker.

**Example chunks:** `['rust is fast', 'rust is safe', 'safe systems']`.

**Expected final counts:**

```text
fast: 1
is: 2
rust: 2
safe: 2
systems: 1
```

**Requirements:** Normalize words to lowercase. Make output order deterministic. Handle a worker panic or channel closure instead of silently losing results.

**Done when:** The parallel result equals a single-threaded reference count and no shared mutable map is used.

**Stretch:** Benchmark several chunk sizes on generated input large enough to make overhead visible.

## Challenge 035 — Concurrent job queue

**Challenge:** A fixed-size worker pool must process every queued job and then shut down without hanging.

**Your task:** Start `2` worker threads and submit jobs `1..=5`. Each worker takes a job, records its ID as completed, and waits for more work until shutdown.

**Expected result:**

```text
Completed job IDs: [1, 2, 3, 4, 5]
Workers stopped: 2
```

Completion order may vary; sort IDs before the final assertion.

**Requirements:** Use `Arc` only for truly shared ownership and protect shared mutable state with `Mutex`. Define a shutdown signal, wake blocked workers, and join them.

**Done when:** Every job appears exactly once and the process exits cleanly on repeated runs.

**Stretch:** Replace the mutex-protected queue with a channel-based design and compare contention and shutdown signaling.

## Challenge 036 — Async status checker

**Challenge:** Check several independent services concurrently and preserve each result even when another request fails or times out.

**Your task:** Use an async runtime and HTTP client to check at least three configurable URLs. Return a status record containing the service name, outcome, and elapsed time.

**Example final report:**

```text
api: 200 OK
search: timeout
billing: request error
```

Actual order may follow completion order, but tests should use a local mock server rather than public endpoints.

**Requirements:** Start checks before awaiting all results. Apply a per-service timeout. A single failure must not cancel or discard the other records.

**Done when:** Three delayed mock responses complete in roughly the longest individual delay, not the sum of all delays.

**Stretch:** Add one deadline for the whole dashboard refresh and mark unfinished checks as cancelled.
