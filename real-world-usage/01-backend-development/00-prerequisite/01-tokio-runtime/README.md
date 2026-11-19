# 01 — Tokio runtime and cancellation

## Goal

Build `jobdeck`, an in-memory concurrent job executor. Producers submit work, bounded workers process it, callers can await results, and the process shuts down without abandoning acknowledged jobs.

Use the [Tokio tutorial](https://tokio.rs/tokio/tutorial), [Tokio crate documentation](https://docs.rs/tokio/latest/tokio/), and [graceful shutdown topic](https://tokio.rs/tokio/topics/shutdown).

## Before you start

Complete the readiness exercise in the [track guide](../README.md) and read the workbook's async chapter through spawning tasks and channels. You need to understand ownership and `Result`; you are not expected to know Tokio's APIs yet. No database, HTTP server, or Docker is used here.

Create a binary package named `jobdeck`. Add Tokio with the macros, multi-threaded runtime, synchronization, signal, and time features. Begin with one `main.rs`; split modules only after the first two checkpoints work.

## Mental model

An async task only makes progress while it is being polled. `.await` yields rather than blocking a runtime thread. Spawning transfers owned state into independently scheduled work; it does not make an unsafe shared reference valid. A bounded channel is both communication and an admission limit. Cancellation usually means a future is dropped, so every `.await` is a point where partially completed work may stop.

For this exercise, an **acknowledged job** is one for which `submit` returned `Accepted`; shutdown must either finish it or report it as canceled. A job rejected because the queue is full was never acknowledged.

## Suggested shape

```text
src/
  main.rs       # demo input and Ctrl-C
  job.rs        # Job, JobId, JobResult, JobError
  executor.rs   # handle, manager loop, workers, shutdown
tests/
  executor.rs
```

Expose a cloneable submission handle rather than the channel sender itself. That keeps admission and shutdown policy inside your API.

## Build it in milestones

### 1. Observe async scheduling

Spawn two named demo tasks that sleep for different durations and return values. Await both handles and print their results. Then temporarily replace one async sleep with `std::thread::sleep` and observe why the runtime stalls when a worker is occupied.

**Checkpoint:** one test proves two async sleeps overlap. Write three sentences explaining why overlap is concurrency, not necessarily parallelism.

### 2. Add a bounded manager

Define `Job`, `JobOutcome`, and `SubmitError`. Run a single manager task that owns the receiver and starts work only while a semaphore permit is available. Send each caller's result through a `oneshot` channel. Use small values such as queue capacity 4 and concurrency 2 so behavior is visible.

**Checkpoint:** submit three deterministic jobs and receive three results; a counter proves no more than two ran together.

### 3. Make admission and deadlines explicit

Use non-waiting admission for the full-queue case. Wrap each job's work in a deadline and distinguish `TimedOut` from `Canceled` and `QueueFull`. Treat a dropped result receiver as normal: the work may finish, but sending its result can fail.

**Checkpoint:** fill the workers and queue deliberately, assert the next submission returns immediately, then use paused Tokio time to advance a job past its deadline.

### 4. Own every task and shut down

Track worker handles in a `JoinSet` or equivalent owner. On cancellation, close admission, drain accepted work until a configured deadline, then abort and join what remains. Return a `ShutdownReport`; do not infer success from log lines.

**Checkpoint:** a short job completes during shutdown, a long job is canceled after the drain deadline, every spawned handle is joined, and the process exits.

### 5. Isolate blocking work

Add one bounded job variant that performs blocking work through `spawn_blocking`. Limit how many such jobs may be admitted; `spawn_blocking` is not itself a capacity policy.

**Checkpoint:** a fast timer continues to fire while the blocking job runs.

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

These are graduation tests after the milestone checkpoints pass.

- `T01` — At most the configured number of jobs run simultaneously.
- `T02` — A full queue rejects admission without waiting indefinitely.
- `T03` — Deadline expiry returns a typed timeout and releases capacity.
- `T04` — Dropping a result receiver does not crash a worker or leak the job record.
- `T05` — Shutdown drains jobs that finish before the deadline and cancels the rest.
- `T06` — Blocking work does not stall an independent timer task.
- `T07` — Paused Tokio time makes timeout and backoff tests deterministic.

## Completion evidence

Submit a task-lifecycle diagram, an explanation of cancellation safety at each `.await`, test output, and a short note distinguishing concurrency from parallelism. You should be able to explain `Send`, task ownership, backpressure, and why dropping a future is observable behavior.
