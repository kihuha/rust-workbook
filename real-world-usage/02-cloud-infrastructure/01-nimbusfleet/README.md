# 01 — Cloud infrastructure: NimbusFleet deployment controller

## Company and service

NimbusFleet is a fictional managed-cloud provider operating compute clusters in three regions. Platform engineers need a service that rolls a container version across worker groups without exceeding disruption limits.

Build `fleet-controller`, a control-plane service plus a lightweight node agent. The controller stores desired state; agents report observed state and execute assigned rollout steps.

## Brown Rust Book prerequisites

Study [modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [traits and lifetimes](https://rust-book.cs.brown.edu/ch10-00-generics.html), [workspaces](https://rust-book.cs.brown.edu/ch14-03-cargo-workspaces.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [async](https://rust-book.cs.brown.edu/ch17-00-async-await.html), and [design tradeoffs](https://rust-book.cs.brown.edu/ch18-05-design-challenge.html).

## Required components

- `fleet-api`: authenticated REST API for operators and CI systems.
- `fleet-controller`: reconciliation loop and rollout state machine.
- `fleet-agent`: node daemon using outbound-only mTLS communication.
- `fleet-store`: durable desired state, leases, events, and audit records.
- `fleetctl`: operator CLI for rollout status and emergency pause.

Use a Cargo workspace with separate crates for protocol types, domain rules, storage, server, agent, and CLI.

## Control-plane API

| Method | Endpoint | Purpose |
| --- | --- | --- |
| `POST` | `/v1/apps` | Register an application and rollout policy |
| `PUT` | `/v1/apps/{app}/desired` | Set image digest and configuration revision |
| `GET` | `/v1/apps/{app}/rollouts/{id}` | Inspect rollout progress and failures |
| `POST` | `/v1/apps/{app}/rollouts/{id}/pause` | Pause new replacements |
| `POST` | `/v1/apps/{app}/rollouts/{id}/resume` | Resume a paused rollout |
| `POST` | `/v1/apps/{app}/rollouts/{id}/rollback` | Restore the previous healthy revision |
| `GET` | `/v1/nodes` | List node heartbeats and assigned revisions |
| `GET` | `/health/ready` | Report storage and leadership readiness |

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

## Deployment target

Deploy three controller replicas across availability zones with one active lease holder. Use a managed relational database, automated migrations, encrypted backups, network policies, pod disruption budgets, and separate agent release artifacts.

Provide Terraform or equivalent infrastructure definitions, dashboards, alerts, certificate-rotation runbooks, disaster recovery steps, and a staged upgrade plan for protocol compatibility.

## Service-level objectives

- API and reconciliation availability: `99.95%` monthly.
- Healthy agent heartbeat reflected in control plane within `15 seconds` p99.
- Controller restart must not lose desired state or issue duplicate non-idempotent actions.
- A 1,000-node rollout must remain within its configured disruption budget.

## Acceptance criteria

The system is complete when an operator can register an app, roll out a digest, observe progress, survive controller and agent restarts, pause, resume, rollback, rotate credentials, and recover the control-plane store.

## Optional extensions

Add canary cohorts, policy-as-code admission, signed configuration bundles, multi-cluster federation, or automatic rollback based on application metrics.
