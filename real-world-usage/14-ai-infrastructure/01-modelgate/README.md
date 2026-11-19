# 01 — AI infrastructure: ModelGate inference gateway

## Business scenario

Product teams call several hosted or locally simulated language models. The platform must stream
responses, enforce tenant budgets, route by policy, survive provider degradation, and show the
latency, quality, and cost consequences of each decision.

Build `modelgate`, a Rust gateway, provider simulator/adapters, policy service, usage ledger, and
evaluation runner. The default project must be runnable without paid APIs or a GPU.

## Problem statement

Route and stream bounded inference work across unreliable providers while enforcing tenant policy,
reconciling usage, measuring quality/latency/cost, and rolling model policies back safely.

## Learning objectives

- implement streaming adapters, cancellation, quotas, and admission control;
- reconcile token/cost usage across ambiguous provider outcomes;
- route with explicit quality, latency, residency, and cost evidence;
- evaluate, shadow, promote, deprecate, and roll back model policy.

## Prerequisites and research

Complete [StreamGate](../../05-networking/01-streamgate/README.md),
[TenantPlane](../../02-cloud-infrastructure/07-tenantplane/README.md), and optionally
[SearchGrid](../../13-search-observability/02-searchgrid/README.md). Review
[Brown Rust Book Chapters 10 and 17](https://rust-book.cs.brown.edu/ch10-00-generics.html),
[vLLM documentation](https://docs.vllm.ai/), [OpenAI production best practices](https://platform.openai.com/docs/guides/production-best-practices),
and the [NIST AI RMF](https://www.nist.gov/itl/ai-risk-management-framework).

## System boundaries and constraints

Use deterministic provider simulators by default. Training, fine-tuning, GPU kernels, factuality
guarantees, end-user chat UI, and paid-provider access are not required for the learning evidence.

## Requirements and guarantees

- Accept versioned completion/chat requests and stream ordered chunks with cancellation.
- Normalize provider capabilities/errors behind explicit adapter contracts.
- Enforce request, concurrency, input/output token, and spend budgets per tenant/project.
- Route by capability, health, latency, quality tier, residency, and configured cost policy.
- Retry or fall back only before/when semantics make duplicate generation acceptable.
- Record immutable usage/cost events and reconcile ambiguous provider outcomes.
- Run a versioned evaluation dataset before promoting a model or routing policy.
- Redact secrets and sensitive prompts from default telemetry and caches.

Quota decisions and authorization must not be bypassed by streaming/retry paths. Usage reconciliation
must not silently undercount. State whether output caching is permitted and how tenant/data/model
versions scope a cache entry. “Correctness” includes evaluation and policy compliance, not factual
perfection of model output.

## Workload and decisions

Use deterministic simulators for fast, slow, rate-limited, malformed, mid-stream failure, and
charged-but-response-lost outcomes. Mix short interactive and long batch prompts across 50 tenants.
Measure time to first token, inter-token gap, end-to-end p50/p95/p99, token throughput, queue time,
fallback rate, wasted tokens, quality score, and estimated cost/request. Compare queue/reject,
static/adaptive routing, immediate/batched execution, and provider/self-hosted economics.

## Milestones

### Senior — safe streaming gateway

Implement adapters, bounded streaming, cancellation, quotas, usage ledger, telemetry, deterministic
failure tests, and one evaluation suite.

### Staff — routing and multi-tenant scheduling

Add capability/policy routing, fallback, admission control, weighted tenant fairness, token-budget
reservation/reconciliation, shadow evaluation, and compatible provider/model changes.

### Principal — model platform strategy

Define quality/latency/cost tiers, residency and retention, build-versus-buy economics, model
deprecation, incident authority, team ownership, capacity investment, and a staged routing migration.

## Required experiments

Cancel at every streaming phase; inject provider `429`, latency, malformed chunks, and ambiguous
billing; exhaust a tenant budget; saturate long prompts beside interactive traffic; shadow a model
change; and roll back a policy. Prove bounded resources, tenant fairness, ledger reconciliation,
evaluation gates, and no secret leakage. Provide provider-degradation, cost-spike, quota, privacy,
bad-model-rollout, and rollback runbooks.

## Operational readiness

Expose admission/queueing, time to first token, inter-token gaps, provider health, fallbacks,
reserved/reconciled usage, tenant fairness, evaluation gates, and cost. Exercise provider disable,
ledger rebuild, credential rotation, policy/model rollback, and privacy/retention response.

## Completion evidence and review

Submit gateway, simulators/adapters, evaluation runner, workload and failure harnesses, quality/
latency/cost report, threat model, routing ADRs, dashboards, alerts, and runbooks. Enforce tenant and
operator authorization, secret handling, prompt/response retention, cache isolation, and egress
policy. Defend the evaluation gate, retry/fallback semantics, quota accounting, fairness, provider
concentration, self-hosted-versus-managed economics, and safe model/policy deprecation.

## Architecture review questions

- When can retry or fallback produce duplicate cost or inconsistent user experience?
- Which evaluation evidence permits policy promotion, and how is regression rolled back?
- Where are prompts, outputs, usage, and caches retained and authorized?
