# Chapter 12 — Concurrency and async/await

## Challenge 034 — Parallel word counts

**Mini brief:** A log analyzer should split independent text chunks across threads and combine word counts.

**Practice:** Threads, ownership transfer, `move`, and message passing.

**Requirements:** Spawn a worker per chunk, send each result back to the main thread through a channel, and combine counts. Join workers and handle possible failures.

**Done when:** No shared mutable map is required and the final counts match a single-threaded check.

**Stretch:** Benchmark different chunk counts on a sufficiently large generated input.

## Challenge 035 — Concurrent job queue

**Mini brief:** A small image service needs a fixed number of workers to process queued jobs.

**Practice:** `Arc`, `Mutex`, thread safety, and coordinating shared state.

**Requirements:** Create a bounded set of worker threads that consume jobs and report completion. Keep shared state minimal and make shutdown behavior explicit.

**Done when:** All submitted jobs are accounted for and workers exit cleanly.

**Stretch:** Use a channel-based queue instead of a shared mutex-protected queue and compare the design.

## Challenge 036 — Async status checker

**Mini brief:** A status dashboard requests several independent service checks and displays results as they finish.

**Practice:** Futures, `async`/`await`, and choosing an async runtime.

**Requirements:** Use a runtime and HTTP client crate to run at least three checks concurrently. Handle timeout and request errors per service so one failure does not discard all results.

**Done when:** Results are gathered concurrently and each service reports success or its own error.

**Stretch:** Add a cancellation deadline for the entire dashboard refresh.
