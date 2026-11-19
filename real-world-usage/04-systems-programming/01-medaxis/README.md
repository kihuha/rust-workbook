# 01 — Systems programming: MedAxis service supervisor

## Company and service

MedAxis is a fictional hospital-device vendor. Its Linux bedside gateway runs local processes for sensor ingestion, alarm routing, audit storage, and encrypted upload. A missed alarm is safety-critical, while uncontrolled restart loops can exhaust a small field device. The gateway therefore needs a supervisor whose failure behavior is explicit, bounded, and diagnosable offline.

Build `medsupervisord` and its client `medsupervisorctl`. The daemon owns child lifecycle, health evaluation, resource containment, and durable diagnostics. The client communicates over a restricted Unix socket. Target recent Linux kernels and document every platform-specific assumption.

This is not a general systemd replacement. It supervises a fixed application stack on one appliance and prioritizes predictable recovery over a large feature surface.

## Operational goals

- Start the complete service graph deterministically after boot.
- Detect a deadlocked or unhealthy service even when its process is still alive.
- Restart faults without creating storms or hiding persistent failure.
- Stop an entire service process tree without signaling an unrelated reused PID.
- Survive daemon crashes and device reboots with enough history for field diagnosis.
- Apply configuration changes without exposing a half-valid desired state.
- Remain within strict CPU, memory, PID, file-descriptor, and disk budgets.

## Brown Rust Book prerequisites

Study [ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [smart pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [patterns](https://rust-book.cs.brown.edu/ch19-00-patterns.html), and [unsafe Rust](https://rust-book.cs.brown.edu/ch20-01-unsafe-rust.html).

## Required behavior

- Load service definitions from `/etc/medsupervisor/services.d/*.toml`.
- Distinguish hard requirements from start ordering and start processes in deterministic graph order.
- Track PID, process start identity, exit status, restart count, and health.
- Apply restart policies: `never`, `on-failure`, and `always`.
- Use exponential backoff with jitter, a restart window, and a circuit breaker.
- Run readiness and liveness probes with bounded concurrency and deadlines.
- Forward configured signals and enforce graceful-stop timeouts.
- Persist state transitions and bounded diagnostic output without treating stale PIDs as owned processes.
- Create one cgroup v2 subtree per service and terminate the whole subtree during cleanup.
- Drop privileges, capabilities, and resource limits before executing a child.
- Reconcile observed state toward desired state after startup, reload, or partial failure.

## Configuration example

```toml
[service.sensor-reader]
command = "/opt/medaxis/bin/sensor-reader"
args = ["--device", "/dev/ttyS1"]
user = "medaxis"
group = "medaxis-sensors"
restart = "on-failure"
restart_limit = 5
restart_window_seconds = 60
restart_backoff = "500ms"
restart_backoff_max = "30s"
stop_timeout_seconds = 10
requires = ["local-audit"]
after = ["device-setup"]
secrets_from = "/run/medaxis/sensor-reader.env"

[service.sensor-reader.resources]
memory_max = "128M"
pids_max = 32
cpu_max = "20000 100000"
open_files = 256

[service.sensor-reader.readiness]
kind = "unix-connect"
path = "/run/medaxis/sensor-reader.sock"
initial_delay = "1s"
period = "2s"
timeout = "500ms"
success_threshold = 1
failure_threshold = 5

[service.sensor-reader.logging]
stdout = "journal"
stderr = "journal"
max_line_bytes = 16384
```

Reject unknown fields, duplicate names, missing requirements, dependency cycles, shell command strings, unsafe executable ownership, invalid users, unbounded probe settings, and paths that escape approved roots.

`requires` means the dependent cannot be healthy unless the requirement is running and ready. `after` controls ordering only. Failure of a requirement moves dependents out of service in reverse dependency order; recovery starts them again in forward order.

## Supervisor architecture

```text
src/
├── config/       # schema, semantic validation, graph construction, reload diff
├── domain/       # service state machine, restart policy, health model
├── supervisor/   # desired/observed reconciliation and dependency coordination
├── process/      # spawn, pidfds, signals, reaping, process groups
├── health/       # probe scheduler, deadlines, thresholds
├── cgroup/       # cgroup v2 creation, limits, metrics, subtree termination
├── control/      # framed protocol, peer authorization, request dispatch
├── journal/      # snapshots, append-only events, bounded child output
└── bin/           # daemon and control client entry points
```

Keep Linux-specific operations behind narrow interfaces. Raw syscalls and any `unsafe` blocks belong in small adapter modules with documented preconditions, not in state-machine or command-handling code. Use a fake monotonic clock, fake process backend, and in-memory journal for deterministic model tests.

## Child spawn contract

Spawning is a security boundary. Define and test the exact parent/child handshake:

1. Resolve and validate the executable, user, group, environment sources, working directory, logging targets, and cgroup limits before creating a child.
2. Create pipes or socket pairs with close-on-exec, allocate the service cgroup, and open required log files.
3. Spawn the child in a new session and process group. The child waits on a synchronization descriptor while the parent records its identity and places it in the cgroup.
4. In the child, set supplementary groups, group ID, user ID, umask, resource limits, capabilities, working directory, and parent-death behavior in a documented safe order.
5. Close every descriptor not explicitly inherited and call `execve` with an argument vector and constructed environment. Never invoke a shell.
6. Report pre-exec failure through a dedicated error pipe. Publish `Starting` only after ownership and identity are known.

Make the supervisor a child subreaper where appropriate. Prefer pidfds for signaling and identity checks; provide a carefully tested `/proc/<pid>/stat` start-time fallback for supported older kernels. Reap every child, including double-forking descendants adopted by the supervisor.

## Local control interface

Expose a permission-restricted Unix domain socket using a versioned, length-prefixed protocol. Every frame carries a request ID and command tag. Enforce a small maximum frame size and deadline before allocating payload memory. Authenticate clients with socket permissions and `SO_PEERCRED` before parsing privileged requests.

| Command | Result |
| --- | --- |
| `list` | Service name, state, PID, uptime, restarts |
| `status <name>` | Full observed and configured state |
| `start <name>` | Start one service and required dependencies |
| `stop <name>` | Graceful stop followed by forced termination on timeout |
| `restart <name>` | Controlled stop and start |
| `reset-failed <name>` | Close the circuit breaker and retry a failed service |
| `events [name]` | Show recent durable state and health events |
| `logs <name>` | Stream bounded child output with cursor-based resume |
| `reload` | Validate and atomically apply configuration changes |

Support stable `--format json` output and meaningful exit codes. A slow log consumer must not block process reaping, health checks, or other control clients.

## State model

Model `Stopped`, `Starting`, `Running`, `Stopping`, `Backoff`, and `Failed` as explicit states. Readiness is an observation within `Starting`; liveness is an observation within `Running`.

| State | Entry cause | Normal exit |
| --- | --- | --- |
| `Stopped` | Initial state, requested stop, or disabled config | Start request or desired-state reconciliation |
| `Starting` | Child spawned but not yet ready | Readiness success to `Running`; exit or timeout to `Backoff`/`Failed` |
| `Running` | Required dependencies and readiness are satisfied | Stop request to `Stopping`; exit or liveness failure to restart handling |
| `Stopping` | Shutdown, dependency loss, reload, or operator request | Child tree reaped to `Stopped`; timeout escalates to cgroup termination |
| `Backoff` | Restart policy permits another attempt | Monotonic deadline to `Starting`; rate limit to `Failed` |
| `Failed` | Circuit breaker open or unrecoverable setup error | Explicit reset or a validated configuration change |

Every transition records its trigger, previous and next state, configuration generation, process identity, and monotonic and wall-clock timestamps. Stale probe results carry an instance generation and cannot change the state of a replacement process.

Avoid PID-reuse bugs: verify the process identity or use safer kernel handles where available. Reap all child processes. Do not run arbitrary shell strings; execute a path and argument vector directly.

## Health and restart semantics

Support `process`, `exec`, `unix-connect`, and `tcp-connect` probes. Probe processes run in the service's cgroup and security context with output and runtime limits. Schedule against a monotonic clock and skip rather than pile up overlapping probes.

A service becomes ready only after its success threshold. It becomes unhealthy only after its failure threshold. Readiness failure prevents dependents from starting; liveness failure requests a controlled restart. An ordinary process exit still uses the configured restart policy.

Restart accounting is based on attempts within a sliding monotonic window. Backoff grows exponentially, includes bounded jitter, and resets only after a configured stable-running period. When the limit is exceeded, open the circuit breaker and require operator action or a relevant configuration change.

## Configuration reload and recovery

Parse all files into an immutable desired-state generation, validate cross-file invariants, and compute a diff before committing it. Invalid input changes nothing. After commit:

- Start added services in dependency order.
- Stop removed services in reverse dependency order.
- Restart services whose execution or security settings changed.
- Apply probe and restart-policy changes without a restart when safe.
- Restart dependents only when their hard requirement contract demands it.

Reload is atomic at the desired-state boundary, but process actions are not instantaneous. Persist the committed generation and reconciliation progress so a daemon crash resumes convergence rather than reverting to an unknowable mix.

Store atomic service snapshots and an append-only, checksummed event journal under `/var/lib/medsupervisor`. Rotate by size while retaining a bounded number of segments. On startup, validate the journal, recover the last complete records, mark previously running instances as interrupted, clean owned cgroups, and start from the committed desired state. Never reattach using a stale numeric PID alone.

## Safety and security

- Minimize and audit any `unsafe` or direct syscall wrapper.
- Validate executable ownership and permissions.
- Prevent configuration paths from escaping allowed roots.
- Authenticate control clients through socket ownership and peer credentials.
- Redact configured secret environment variables.
- Apply least-privilege users, groups, capabilities, and resource limits.
- Keep secrets out of configuration, process listings, logs, core dumps, and client responses.
- Refuse writable configuration or executable files owned by unauthorized users.
- Bound protocol frames, log lines, retained events, probe output, and in-memory queues.

## Observability

Write structured local logs with service, instance generation, process identity, and configuration generation. Expose restart counts, uptime, readiness and liveness state, transition counts, health failures, probe latency, spawn latency, resource pressure, dropped log lines, queue depth, journal errors, and forced kills. Rotate files safely or integrate with the system journal.

Provide a single diagnostic bundle command that collects redacted configuration, recent events, current state, kernel and cgroup information, and bounded logs without requiring network access.

## Delivery stages

1. **Lifecycle core:** typed configuration, state machine, deterministic dependency graph, direct `exec`, process groups, pidfds, signal handling, and child reaping.
2. **Recovery:** restart policy, monotonic backoff, circuit breaker, atomic snapshots, checksummed journal, and startup reconciliation.
3. **Health and containment:** readiness/liveness probes, cgroup v2 limits, privilege dropping, capabilities, and whole-tree termination.
4. **Operations:** authenticated control protocol, logs and events, atomic desired-state reload, diagnostic bundles, packaging, and fault-injection soak tests.

## Required tests

### Unit tests

- `T01` — Every allowed state transition succeeds and every forbidden transition is rejected.
- `T02` — Restart backoff and rate limits follow a fake monotonic clock.
- `T03` — Dependency sorting rejects cycles and missing services.
- `T04` — Config reload is all-or-nothing when one file is invalid.
- `T05` — Stale health results cannot affect a replacement process generation.
- `T06` — Restart jitter remains bounded and a stable interval resets the circuit breaker counter.

### Integration tests

- `T07` — Start, inspect, stop, and reap a child and its orphaned descendant.
- `T08` — A crashing child reaches `Failed` after the restart limit.
- `T09` — Graceful stop escalates only after the configured timeout and empties the cgroup.
- `T10` — Readiness gates a dependent and liveness failure triggers controlled restart.
- `T11` — Socket authorization rejects an unauthorized local user before command dispatch.
- `T12` — A slow log client cannot block reaping or health evaluation.
- `T13` — A valid reload converges additions, removals, and restarts in safe graph order.
- `T14` — Supervisor restart does not attach to an unrelated reused PID.
- `T15` — Truncated journal tails recover to the last checksummed event.

### System and resilience tests

- `T16` — Enforce memory, CPU, PID, and file-descriptor limits without affecting the supervisor.
- `T17` — Kill the daemon after each spawn and reload step, then verify clean recovery.
- `T18` — Reboot a VM during a restart storm and verify bounded recovery.
- `T19` — Exhaust disk space and confirm that supervision continues while diagnostics report degradation.
- `T20` — Fuzz configuration, journal, and control-protocol decoders.
- `T21` — Run sanitizers and Miri where supported for low-level components.

## Deployment target

Ship signed architecture-specific packages with a dedicated system user, configuration directory, Unix socket policy, log policy, and uninstall behavior. Provide a systemd unit only for launching the supervisor itself.

Document supported kernels and architectures, upgrades, configuration compatibility, rollback, core-dump handling, and field diagnostics for offline devices.

## Acceptance criteria

The supervisor boots a fixture service graph, gates dependents on readiness, contains resource abuse, recovers from crashes, applies a valid reload, rejects an invalid reload, and produces an offline diagnostic bundle. It runs for 72 hours under restart, probe, disk, process, and daemon fault injection without zombies, unbounded restart loops, leaked descriptors, incorrect PID ownership, unbounded queues, or silent loss of diagnostic state.

## System-design practice

Declare child count, restart patterns, output rates, health intervals, configuration size, and OS
resource limits. Measure spawn/restart p95/p99, detection delay, memory, descriptor use, and behavior
in a crash loop. Senior proves safe supervision on one host; Staff adds dependency ordering,
backoff budgets, compatible reload, cgroup/resource isolation, and upgrade handoff; Principal defines
fleet control, ownership, security policy, cost, rollout, and rollback. Compare process-per-service
with containers and system managers using the [systemd documentation](https://systemd.io/).

## Optional extensions

Add seccomp profiles, watchdog hardware integration, socket activation, zero-downtime handoff, an immutable signed remote configuration bundle, or an A/B supervisor upgrade mechanism.
