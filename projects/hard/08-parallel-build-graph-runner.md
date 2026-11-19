# Hard 08 — Parallel build graph runner

## Project challenge

Build a task runner that executes a dependency graph in parallel. The challenge is to schedule ready work, detect invalid graphs, cache results, and stop dependents after failure.

## Required behavior

- Load task names, commands, inputs, outputs, and dependencies from a config file.
- Reject missing dependencies and cycles with a useful cycle path.
- Run independent tasks concurrently up to a worker limit.
- Skip tasks whose inputs, command, and dependency outputs have not changed.
- Capture per-task output and print a deterministic final summary.
- Mark downstream tasks blocked when a dependency fails.

## Example graph

```text
generate -> compile -> test
                 \-> package
lint ----------------> package
```

`generate` and `lint` may start together. `package` starts only after both `compile` and `lint` succeed.

## Architecture expectations

Separate graph validation, readiness scheduling, process execution, fingerprinting, cache storage, and reporting. Avoid holding scheduler locks while a process runs.

## Rust skills combined

Directed graphs, cycle detection, concurrent scheduling, process management, hashing, caching, shared state, cancellation, and scenario tests.

## Done when

Every task runs at most once, cycles are rejected before execution, concurrency respects the limit, failure blocks only dependent work, and cache invalidation is tested.

## System-design foundation

Make desired dependencies, observed results, cache fingerprints, and retry/cancellation state
explicit. Measure critical-path time and worker utilization. This prepares reconciliation loops,
workflow orchestration, and fair scheduling.

### Foundation evidence

- `F01` — Inject task failure/cancellation/cache corruption and prove only valid ready work runs and dependents reach explicit states.
- `F02` — Benchmark serial and bounded parallel execution; report critical path, utilization, cache hit rate, and peak runnable queue.

## Brown Rust Book prerequisites

Study [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [closures and iterators](https://rust-book.cs.brown.edu/ch13-00-functional-features.html), [worker coordination](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), and [pattern syntax](https://rust-book.cs.brown.edu/ch19-03-pattern-syntax.html).

## Suggested tests

### Unit tests

- `T01` — Reject a missing dependency.
- `T02` — Detect a cycle and report a useful cycle path.
- `T03` — Readiness requires every dependency to succeed.
- `T04` — Fingerprints change when commands, inputs, or dependency outputs change.

### Integration tests

- `T05` — Independent tasks overlap without exceeding the worker limit.
- `T06` — A failed task blocks only its downstream tasks.
- `T07` — A second unchanged run uses the cache and executes no task commands.

## Stretch goals

Add cancellation, remote cache storage, watch mode, or critical-path reporting.
