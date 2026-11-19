# 01 — Cloud infrastructure: NimbusFleet deployment controller

## Company and service

NimbusFleet is a fictional managed-cloud provider operating compute clusters in three regions. Platform engineers need a service that rolls a container version across worker groups without exceeding disruption limits.

Build `fleet-controller`, a control-plane service plus a lightweight node agent. The controller stores desired state; agents report observed state and execute assigned rollout steps.

## Brown Rust Book prerequisites

Study [modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [traits and lifetimes](https://rust-book.cs.brown.edu/ch10-00-generics.html), [workspaces](https://rust-book.cs.brown.edu/ch14-03-cargo-workspaces.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [async](https://rust-book.cs.brown.edu/ch17-00-async-await.html), and [design tradeoffs](https://rust-book.cs.brown.edu/ch18-05-design-challenge.html).

## Required components

- `fleet-control`: authenticated operator/CI boundary implemented as a CLI over a local socket,
  HTTP, gRPC, or another justified adapter.
- `fleet-controller`: reconciliation loop and rollout state machine.
- `fleet-agent`: node daemon using outbound-only mTLS communication.
- `fleet-store`: durable desired state, leases, events, and audit records.
- `fleetctl`: operator CLI for rollout status and emergency pause.

Use a Cargo workspace with separate crates for protocol types, domain rules, storage, server, agent, and CLI.

## Control-plane contract

Provide one documented operator/automation interface. The transport is an architectural choice, not
the lesson: a local CLI/socket is sufficient for the Senior milestone; HTTP or gRPC is appropriate
only when the learner wants to practise remote multi-client control. Keep the application service
independent from the chosen adapter.

| Operation | Purpose | Contract requirement |
| --- | --- | --- |
| Register application | Store an application and rollout policy | Validate identity and policy before mutation |
| Set desired revision | Set image digest and configuration revision | Idempotent or compare-and-set; audit every accepted change |
| Inspect rollout | Read progress, failures, and current/desired revisions | Stable machine-readable state, not log scraping |
| Pause/resume | Control whether new replacements may start | Repeated commands are safe |
| Roll back | Restore the previous healthy revision | Name the exact target revision and audit the actor |
| Inspect nodes | List heartbeats, capabilities, and assigned revisions | Paginate/bound results and mark stale observations |
| Read readiness | Report storage, leadership, and reconciliation readiness | Separate process liveness from ability to make safe progress |

If the chosen adapter is remote, authenticate and authorize it and add transport-specific boundary
tests. Do not spend the project implementing a general-purpose API framework.

## Agent protocol

Use streaming gRPC or a documented framed protocol over mTLS. Agents send node ID, boot ID, capabilities, running revisions, health, and monotonic sequence number. Controllers send idempotent commands with deadlines.

```text
Agent -> Heartbeat(node-17, boot-8, running=v42, seq=981)
Controller -> Apply(command-771, desired=v43, deadline=30s)
Agent -> Result(command-771, Healthy, observed=v43)
```

## Rollout rules

- Identify images by immutable digest, never a mutable tag.
- Enforce `max_unavailable` and `max_surge` across each worker group.
- Do not advance a rollout until the new revision passes health for a stabilization window.
- Pause automatically when failure rate crosses the configured threshold.
- Treat agent commands as idempotent across disconnects and retries.
- Use a lease so only one active controller reconciles a rollout at a time.
- Record every desired-state change and operator action in an append-only audit log.

## Failure scenarios

Handle controller restarts, leader loss, agent disconnects, duplicate messages, reordered reports, node replacement, unavailable regions, failed health checks, and storage timeouts.

The reconciliation loop must derive the next action from desired and observed state rather than relying on in-memory progress alone.

## Security

- Authenticate operators with short-lived identity tokens and scoped roles.
- Issue each agent a unique renewable mTLS identity.
- Authorize an agent only for its registered node ID.
- Verify image digests and optional signatures before applying.
- Redact tokens, certificates, environment secrets, and application config from logs.
- Maintain an auditable certificate rotation and revocation path.

## Observability

Expose reconciliation duration, queue depth, connected agents, stale nodes, rollout progress, failed commands, lease state, and per-region health. Add traces spanning API mutation, reconciliation decision, agent command, and result.

## Required tests

### Unit tests

- `T01` — The planner never exceeds `max_unavailable` or `max_surge`.
- `T02` — Duplicate or stale agent sequence numbers do not regress observed state.
- `T03` — Failure threshold crossing changes the rollout to `Paused` exactly once.
- `T04` — A reconciliation run over identical state emits no new command.

### Integration tests

- `T05` — Controller state survives restart during an active rollout.
- `T06` — Two controllers contend for a lease and only the holder issues commands.
- `T07` — Agent reconnect and command replay do not apply a revision twice.
- `T08` — Certificate revocation prevents an agent from reconnecting.

### End-to-end and resilience tests

- `T09` — Roll `v42 -> v43` across 20 simulated nodes while respecting disruption limits.
- `T10` — Inject failed health checks and verify automatic pause and manual rollback.
- `T11` — Partition one region, continue safe progress elsewhere, then reconcile on recovery.
- `T12` — Load-test heartbeats and commands at the target fleet size without unbounded memory growth.

## Local deployment and production design exercise

Run three local controller processes, a local relational database, and simulated agents with one
active lease holder. Demonstrate migration, backup/restore, certificate rotation, controller/agent
version skew, rollout, and rollback in the harness.

Then produce—but do not claim to have validated—a multi-zone deployment proposal covering failure
domains, network policy, disruption budgets, encrypted backups, infrastructure definition,
dashboards, alerts, disaster recovery, and protocol upgrade. Cloud resources are optional.

## Local scale and recovery targets

- Healthy agent heartbeat reflected in control plane within `15 seconds` p99.
- Controller restart must not lose desired state or issue duplicate non-idempotent actions.
- A 1,000-node rollout must remain within its configured disruption budget.

Run these targets with simulated nodes under a declared machine and workload. They establish local
behavior, not production availability. A later architecture review may derive an SLI/objective and
error budget from user impact, dependency behavior, and an actual deployment model.

## Acceptance criteria

The system is complete when an operator can register an app, roll out a digest, observe progress, survive controller and agent restarts, pause, resume, rollback, rotate credentials, and recover the control-plane store.

## System-design progression

The required build is the Senior milestone. Define fleet size, heartbeat distribution, rollout
mix, node churn, configuration size, and storage limits. Measure reconciliation and command
p50/p95/p99, stale-node detection, controller queues, database saturation, and recovery time.

For Staff practice, shard reconciliation ownership, introduce protocol/version skew, fence stale
controllers at the protected agent operation, throttle regional rollout work, and migrate between
planner algorithms while a rollout is active.

For Principal practice, define which actions a region may take while isolated from the central
plane, quorum and control-plane placement, capacity reserve, RPO/RTO, certificate and protocol
deprecation, team ownership, and a disaster-recovery exercise with measured outcomes.

Review [Kubernetes controller guidance](https://kubernetes.io/docs/concepts/architecture/controller/),
the [Raft material](https://raft.github.io/), and [Google SRE](https://sre.google/books/); implement
the relevant behavior rather than copying a production control plane wholesale.

## Optional extensions

Add canary cohorts, policy-as-code admission, signed configuration bundles, multi-cluster federation, or automatic rollback based on application metrics.
