# 01 — Tokio runtime and cancellation

## Goal

Build `jobdeck`, an in-memory concurrent job executor. Producers submit work, bounded workers process it, callers can await results, and the process shuts down without abandoning acknowledged jobs.

Use the [Tokio tutorial](https://tokio.rs/tokio/tutorial), [Tokio crate documentation](https://docs.rs/tokio/latest/tokio/), and [graceful shutdown topic](https://tokio.rs/tokio/topics/shutdown).

## Required behavior

- Accept jobs containing an ID, duration, deadline, and deterministic result instruction.
- Use a bounded `tokio::sync::mpsc` queue and a fixed concurrency limit.
- Return `QueueFull` immediately when admission capacity is exhausted.
- Track completion through `oneshot` channels without retaining abandoned callers forever.
- Apply per-job deadlines with `tokio::time::timeout`.
- Accept shutdown from Ctrl-C or an internal cancellation signal.
- Stop admission, notify all tasks, drain accepted work to a deadline, then abort only remaining tasks.
- Report accepted, completed, timed-out, rejected, and canceled totals.

## Constraints

- Do not hold a synchronous or async mutex guard across `.await` unless the protected type and design require it and you document why.
- Do not call blocking filesystem, CPU-heavy, or sleep functions on a runtime worker. Demonstrate `spawn_blocking` for one bounded blocking operation.
- Every spawned task must have an owner, completion path, and shutdown behavior. Detached tasks are forbidden.
- Compare a single-owner manager task using channels with shared `Arc<Mutex<_>>` state and justify the chosen design.

## Required tests

- `T01` — At most the configured number of jobs run simultaneously.
- `T02` — A full queue rejects admission without waiting indefinitely.
- `T03` — Deadline expiry returns a typed timeout and releases capacity.
- `T04` — Dropping a result receiver does not crash a worker or leak the job record.
- `T05` — Shutdown drains jobs that finish before the deadline and cancels the rest.
- `T06` — Blocking work does not stall an independent timer task.
- `T07` — Paused Tokio time makes timeout and backoff tests deterministic.

## Completion evidence

Submit a task-lifecycle diagram, an explanation of cancellation safety at each `.await`, test output, and a short note distinguishing concurrency from parallelism. You should be able to explain `Send`, task ownership, backpressure, and why dropping a future is observable behavior.
