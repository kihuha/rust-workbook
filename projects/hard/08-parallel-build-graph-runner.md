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

## Stretch goals

Add cancellation, remote cache storage, watch mode, or critical-path reporting.
