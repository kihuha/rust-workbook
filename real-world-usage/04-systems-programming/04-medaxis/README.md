# 04 — Systems programming: MedAxis service supervisor

## Company and service

MedAxis is a fictional hospital-device vendor. Its Linux bedside gateway runs several local processes for sensor ingestion, alarm routing, and encrypted upload. Field devices need a small supervisor that starts services, monitors them, and recovers safely without systemd-specific application logic.

Build `medsupervisord` and its client `medsupervisorctl`. Target recent Linux kernels and document every platform-specific assumption.

## Brown Rust Book prerequisites

Study [ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [smart pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [patterns](https://rust-book.cs.brown.edu/ch19-00-patterns.html), and [unsafe Rust](https://rust-book.cs.brown.edu/ch20-01-unsafe-rust.html).

## Required behavior

- Load service definitions from `/etc/medsupervisor/services.d/*.toml`.
- Start processes in dependency order.
- Track PID, process start identity, exit status, restart count, and health.
- Apply restart policies: `never`, `on-failure`, and `always`.
- Use exponential backoff and a restart-rate limit.
- Forward configured signals and enforce graceful-stop timeouts.
- Persist enough state for post-reboot diagnosis without treating stale PIDs as owned processes.
- Drop privileges and set resource limits before executing a child.

## Configuration example

```toml
[service.sensor-reader]
command = "/opt/medaxis/bin/sensor-reader"
args = ["--device", "/dev/ttyS1"]
user = "medaxis"
restart = "on-failure"
restart_limit = 5
restart_window_seconds = 60
stop_timeout_seconds = 10
depends_on = ["local-audit"]
```

## Local control interface

Expose a permission-restricted Unix domain socket using a length-prefixed protocol.

| Command | Result |
| --- | --- |
| `list` | Service name, state, PID, uptime, restarts |
| `status <name>` | Full observed and configured state |
| `start <name>` | Start one service and required dependencies |
| `stop <name>` | Graceful stop followed by forced termination on timeout |
| `restart <name>` | Controlled stop and start |
| `reload` | Validate and atomically apply configuration changes |

## State model

Model `Stopped`, `Starting`, `Running`, `Stopping`, `Backoff`, and `Failed` as explicit states. Every transition must state its trigger and permitted successor states.

Avoid PID-reuse bugs: verify the process identity or use safer kernel handles where available. Reap all child processes. Do not run arbitrary shell strings; execute a path and argument vector directly.

## Safety and security

- Minimize and audit any `unsafe` or direct syscall wrapper.
- Validate executable ownership and permissions.
- Prevent configuration paths from escaping allowed roots.
- Authenticate control clients through socket ownership and peer credentials.
- Redact configured secret environment variables.
- Apply least-privilege users, groups, capabilities, and resource limits.

## Observability

Write structured local logs with service and process identity. Expose restart counts, uptime, state transitions, health failures, spawn latency, and forced kills. Rotate logs or integrate with the system journal.

## Required tests

### Unit tests

- `T01` — Every allowed state transition succeeds and every forbidden transition is rejected.
- `T02` — Restart backoff and rate limits follow a fake monotonic clock.
- `T03` — Dependency sorting rejects cycles and missing services.
- `T04` — Config reload is all-or-nothing when one file is invalid.

### Integration tests

- `T05` — Start, inspect, stop, and reap a controlled child process.
- `T06` — A crashing child reaches `Failed` after the restart limit.
- `T07` — Graceful stop escalates only after the configured timeout.
- `T08` — Socket authorization rejects an unauthorized local user.
- `T09` — Supervisor restart does not attach to an unrelated reused PID.

### System and resilience tests

- `T10` — Run under memory, CPU, and file-descriptor limits.
- `T11` — Reboot a VM during a restart storm and verify clean recovery.
- `T12` — Fuzz configuration and control-protocol decoders.
- `T13` — Run sanitizers and Miri where supported for low-level components.

## Deployment target

Ship signed architecture-specific packages with a dedicated system user, configuration directory, Unix socket policy, log policy, and uninstall behavior. Provide a systemd unit only for launching the supervisor itself.

Document supported kernels and architectures, upgrades, configuration compatibility, rollback, core-dump handling, and field diagnostics for offline devices.

## Acceptance criteria

The supervisor runs for 72 hours under fault injection without zombies, unbounded restart loops, leaked descriptors, incorrect PID ownership, or loss of diagnostic state.

## Optional extensions

Add readiness probes, cgroup v2 placement, seccomp profiles, watchdog integration, or an immutable remote configuration bundle.
