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

## Stretch goals

Add recurring schedules, dependencies between jobs, or an HTTP control API.
