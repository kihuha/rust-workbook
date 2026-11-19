# 01 — Real-time collaboration: CollabPad editor

## Business scenario

Distributed teams edit shared documents from unreliable networks and multiple devices. Users need
fast local feedback, eventual convergence, presence, reconnect, and an auditable document history.

Build `collabpad`: a Rust WebSocket service, persistence layer, protocol test client, and a minimal
browser or terminal editor. The UI is not the lesson; synchronization and failure behavior are.

## Problem statement

Keep authorized replicas of a shared document convergent through concurrent edits, reconnect, slow
clients, offline work, compaction, and service restart without unbounded session state.

## Learning objectives

- implement bounded real-time sessions, fan-out, presence, and resume;
- select and test a sequencing or merge model for concurrent edits;
- separate ephemeral connection state from durable document history;
- evolve snapshots and protocols while clients remain active.

## Prerequisites and research

Complete the [async chat server](../../../projects/hard/01-async-chat-server.md) and
[event-sourced order system](../../../projects/hard/10-event-sourced-order-system.md). Review
[Brown Rust Book Chapters 15–17](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), the
[CRDT survey](https://inria.hal.science/hal-00932836/document), and
[WebSocket RFC 6455](https://www.rfc-editor.org/rfc/rfc6455).

## System boundaries and constraints

Support plain-text documents and a minimal client. Rich-text semantics, comments, media, general
offline databases, moderation UI, and internet-scale regional deployment are outside the first build.

## Requirements and invariants

- Authenticate a session, open a versioned document, submit operations, and stream remote changes.
- Maintain ephemeral presence separately from durable document state.
- Resume from a known version or return a snapshot plus subsequent operations.
- Bound per-connection queues and disconnect or degrade slow consumers predictably.
- Deduplicate retried client operations and preserve authorized edit history.
- Compact history into snapshots without invalidating connected or offline clients.
- Support concurrent insert/delete edits and define cursor/presence behavior.

All accepted operations must converge to the same document under the chosen model. A user may edit
only an authorized document. Presence expiry must not alter durable content. State whether the
system uses central sequencing, operational transformation, or a CRDT and what it does not support.

## Workload and decisions

Simulate 1,000 connections, 100 active documents, one 200-editor hot document, mobile reconnects,
and out-of-order/duplicate frames. Scale the numbers to local hardware but preserve ratios. Measure
local acknowledgement and fan-out p50/p95/p99, convergence time, queue memory, reconnect bytes,
snapshot pause, and hot-document CPU. Compare centralized sequencing with decentralized merge,
operation history with snapshots, and per-document actors with shared concurrent state.

## Milestones

### Senior — sequenced online editing

Build one service with durable operations, reconnect, bounded fan-out, authorization, presence,
snapshots, and deterministic concurrent-edit tests.

### Staff — offline and horizontally scaled sessions

Add the justified merge model, multi-device cursors, offline changes, multiple gateways, document
ownership/rebalance, hot-document protection, and rolling protocol compatibility.

### Principal — collaboration platform evolution

Define regional routing, residency, tiered limits, moderation/audit boundaries, storage cost,
ownership, and migration from the original sequencing model without corrupting active documents.

## Required experiments

- Generate concurrent edits in different orders and assert convergence.
- Disconnect before/after acknowledgement and prove retry deduplication.
- Make one client slow and one document hot; verify bounded memory and chosen degradation.
- Restart gateways and document owners while clients reconnect.
- Compact during active/offline clients and verify resume or explicit resync.
- Run old/new protocol clients through a staged rollout and rollback.

Provide telemetry and runbooks for connection storms, hot documents, convergence alarms, stuck
presence, snapshot failure, authorization incidents, and failed migration.

## Operational readiness

Expose active/reconnecting sessions, per-document operations and queues, fan-out delay, convergence
alarms, snapshot/compaction state, authorization failures, and hot documents. Exercise gateway and
owner restart, snapshot restore, protocol rollout, migration reversal, and privacy deletion.

## Completion evidence and review

Submit the service and test client/UI, generated concurrency/convergence suite, reconnect and load
harnesses, benchmark report, threat model, protocol/merge ADRs, migration evidence, and runbooks.
Enforce document authorization on every operation and resume, limit content/frame/history sizes,
and define retention/privacy. Defend the merge model, ordering boundary, offline limitations,
hot-document strategy, regional evolution, and why the chosen complexity is justified.

## Architecture review questions

- Why does the chosen sequencing/merge model converge under the tested reorderings?
- What state is ephemeral, authoritative, or rebuildable?
- How can snapshots/protocols evolve without stranding offline clients?
