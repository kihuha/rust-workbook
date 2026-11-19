# Hard 03 — Durable job scheduler

## Project challenge

Build a scheduler that runs persisted jobs at or after their due time. The challenge is to coordinate timing, retries, worker limits, cancellation, and crash recovery.

## Required behavior

- Schedule a command with an ID, due time, timeout, and retry policy.
- Persist every job state transition.
- Run at most a configured number of jobs concurrently.
- Capture exit status and bounded stdout and stderr.
- Retry failures with exponential backoff.
- Recover jobs left in `Running` after an unclean shutdown according to a documented policy.

## Example lifecycle

```text
job-7 Scheduled for 10:00
job-7 Running attempt 1
job-7 RetryWaiting until 10:00:02
job-7 Running attempt 2
job-7 Succeeded
```

## Architecture expectations

Model states with an enum and validate allowed transitions. Separate the clock, persistence, process runner, scheduling queue, and worker coordination so they can be tested independently.

## Rust skills combined

Async scheduling, priority queues, process management, state machines, persistence, retry policies, cancellation, traits, and deterministic tests.

## Done when

Jobs do not exceed the worker limit, cancellation is race-safe, restart reconstructs state, and tests use a fake clock rather than waiting in real time.

## System-design foundation

Separate admission, scheduling, execution, and outcome; use an injected clock; bound runnable work;
and model recovery from every persisted state. This is the direct prerequisite for
[DurableQueue](../../real-world-usage/02-cloud-infrastructure/05-durablequeue/README.md).

### Foundation evidence

- `F01` — Crash after every persisted job state and prove restart reaches the documented eligible terminal/retry state.
- `F02` — Saturate the worker limit with slow jobs; report queue age, start-delay p95/p99, memory, and cancellation behavior.

## Brown Rust Book prerequisites

Study [state enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [trait boundaries](https://rust-book.cs.brown.edu/ch10-02-traits.html), [thread coordination](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), and [tasks versus threads](https://rust-book.cs.brown.edu/ch17-06-futures-tasks-threads.html).

## Suggested tests

### Unit tests

- `T01` — Reject every invalid job-state transition.
- `T02` — Exponential backoff produces the documented retry times.
- `T03` — Cancellation is idempotent or returns a documented error.
- `T04` — The fake clock makes a due job ready without sleeping.

### Integration tests

- `T05` — Running jobs never exceed the configured worker limit.
- `T06` — Restart recovers jobs left in `Running` according to policy.
- `T07` — Timeout terminates the child process and records its outcome.

## Stretch goals

Add recurring schedules, dependencies between jobs, or an HTTP control API.
