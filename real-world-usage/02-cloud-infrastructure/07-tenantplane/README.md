# 07 — Cloud infrastructure: TenantPlane SaaS control plane

## Business scenario

An internal platform provisions isolated workspaces for hundreds of product teams. Tenants have
different sizes, plans, residency requirements, and service expectations. A large import from one tenant must not
starve interactive work for everyone else.

Build `tenantplane`, a Rust control plane plus simulated data-plane workers. Focus on lifecycle,
fairness, compatibility, metering, and ownership rather than a feature-rich user interface.

## Problem statement

Operate tenant lifecycle and shared capacity so failures resume safely, authorization remains
tenant-scoped, and large workloads cannot make small tenants unusable.

## Learning objectives

- implement recoverable control-plane workflows and compatibility checks;
- enforce tenant identity, hierarchical quotas, and fair scheduling;
- reconcile metering and lifecycle state after retries and crashes;
- evolve pooled and isolated placement with measured cost and rollback.

## Prerequisites and research

Complete [DurableQueue](../05-durablequeue/README.md), then review
[Brown Rust Book Chapters 10, 16, and 17](https://rust-book.cs.brown.edu/ch10-00-generics.html),
[AWS Well-Architected SaaS guidance](https://docs.aws.amazon.com/wellarchitected/latest/saas-lens/saas-lens.html),
and the [SRE books](https://sre.google/books/).

## System boundaries and constraints

Data-plane workers are deterministic simulators. Billing produces auditable exercise records, not
real invoices. Product UI, tax, payment collection, and production identity-provider setup are out.

## Required behavior and guarantees

- Create, suspend, resume, migrate, and delete a tenant through recoverable workflows.
- Manage versioned tenant configuration and data-plane capability compatibility.
- Enforce hierarchical quotas for organization, tenant, workload class, and operation.
- Schedule interactive and batch work with a documented fairness policy.
- Meter accepted work without double billing after retries.
- Enforce tenant identity in the query/repository layer and encrypt tenant-sensitive configuration.
- Expose lifecycle progress, quota usage, throttling reason, metering corrections, and audit history.

Tenant deletion must eventually remove scoped data while preserving only documented audit facts.
A failed workflow resumes from durable state. Configuration is applied only to compatible workers.
One tenant cannot address another tenant's resources even with guessed identifiers.

## Workload and decisions

Simulate 200 tenants: 160 small, 35 medium, and 5 large. One large tenant produces 40% of batch
load; interactive work is bursty. Define local latency and throughput targets per tier, a maximum fairness
lag for small tenants, and a storage/concurrency budget. Compare shared, pooled, and siloed tenant
placement; token buckets with deficit/weighted scheduling; synchronous with workflow-based
provisioning; and hard with elastic quotas.

## Milestones

### Senior — tenant-safe control plane

Implement lifecycle workflows, repository-level isolation, quotas, audit, metering, and restart
recovery against simulated workers.

### Staff — fairness and compatibility

Add hierarchical admission, workload classes, noisy-neighbor experiments, worker version skew,
configuration rollout, tenant moves, dashboards, and alerts tied to the measured targets.

### Principal — platform strategy

Design tier economics, regional placement, residency policy, team ownership, an API deprecation
program, capacity investment, and a staged migration from pooled to selectively isolated tenants.

## Required experiments

- Attempt cross-tenant access through every identifier-bearing API and worker message.
- Saturate batch capacity with one tenant and verify interactive and small-tenant bounds.
- Crash each lifecycle step and resume without duplicate resources or billing.
- Roll out an incompatible configuration and prove admission blocks unsafe application.
- Move a tenant while reads/writes continue, verify completeness, and exercise rollback.
- Rebuild invoices from immutable usage events and reconcile corrections.

Report fairness, throttle rate, lifecycle duration, configuration skew, per-tenant resource use, and
estimated cost. Provide runbooks for noisy tenants, stuck workflows, metering disagreement,
compromised credentials, failed migration, and deletion backlog.

## Operational readiness

Expose workflow age, quota/fairness decisions, worker compatibility, metering lag, tenant moves,
deletion backlog, and per-tenant resource use. Exercise credential response, workflow recovery,
configuration rollback, tenant move reversal, metering rebuild, and deletion verification.

## Completion evidence and review

Submit the control plane and simulator, isolation/property tests, workload generator, fairness and
cost report, threat model, lifecycle ADRs, migration evidence, dashboards, alerts, and runbooks.
Defend shared-versus-silo placement, the fairness objective, quota hierarchy, deletion guarantee,
metering source of truth, tenant move rollback, and which tier promises justify extra complexity.

## Architecture review questions

- Which resources are shared, pooled, or siloed, and why?
- How is fairness defined and verified when workloads have different costs?
- Which tenant lifecycle step is hardest to reverse safely?
load; interactive work is bursty. Define local latency and throughput targets per tier, a maximum fairness
