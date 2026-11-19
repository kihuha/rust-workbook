# 01 — Blockchain: Provena supply-chain registry

## Company and service

Provena is a fictional food-export consortium. Producers, laboratories, shippers, and retailers need a shared record of custody and certification events across organizations that do not share one database administrator.

Build a Rust-based smart contract or runtime module plus an off-chain indexer and API. The ledger stores hashes and identifiers; confidential documents remain in controlled object storage.

## Brown Rust Book prerequisites

Study [ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [enums and matching](https://rust-book.cs.brown.edu/ch06-00-enums.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [tests](https://rust-book.cs.brown.edu/ch11-00-testing.html), and [advanced features](https://rust-book.cs.brown.edu/ch20-00-advanced-features.html).

## Actors and permissions

- Consortium administrators approve and suspend organizations.
- Producers create product batches.
- Laboratories attach certification hashes and expiry dates.
- Custodians transfer possession.
- Retailers mark batches received or recalled.
- Auditors read complete public event history.

Use role-based authorization on-chain. Every mutation must identify the signer, organization, block height, and event sequence.

## On-chain model

- `Organization`: ID, role set, public signing identity, active state.
- `Batch`: ID, product code, producer, production timestamp, current custodian, state.
- `Certification`: issuer, document hash, schema, issued and expiry timestamps.
- `CustodyTransfer`: from, intended recipient, status, timestamps.
- `Recall`: reason code, evidence hash, affected scope, issuer.

Define explicit states such as `Created`, `InTransit`, `Received`, `Recalled`, and `Closed`. Reject impossible or unauthorized transitions.

## Required contract calls

| Call | Purpose |
| --- | --- |
| `register_organization` | Add an approved consortium participant |
| `create_batch` | Create a globally unique batch |
| `attach_certification` | Record a hash and validity metadata |
| `initiate_transfer` | Name the next intended custodian |
| `accept_transfer` | Complete transfer by the intended recipient |
| `recall_batch` | Mark a batch and descendants recalled |
| `close_batch` | Prevent further custody changes |

## Emitted events

```text
BatchCreated(batch_id, producer, sequence)
TransferInitiated(batch_id, from, to, sequence)
TransferAccepted(batch_id, custodian, sequence)
CertificationAttached(batch_id, issuer, document_hash, sequence)
BatchRecalled(batch_id, issuer, reason_code, sequence)
```

Event schema changes require explicit versioning. The indexer must tolerate chain reorganization or finality according to the selected platform.

## Off-chain API

| Method | Endpoint | Purpose |
| --- | --- | --- |
| `GET` | `/v1/batches/{id}` | Materialized batch state and finality |
| `GET` | `/v1/batches/{id}/history` | Ordered provenance events |
| `GET` | `/v1/batches/{id}/certifications` | Certification metadata and verification state |
| `GET` | `/v1/organizations/{id}` | Public participant status |
| `POST` | `/v1/documents/verify` | Compare uploaded bytes to an on-chain hash without storing them |
| `GET` | `/v1/indexer/status` | Indexed height, finalized height, and lag |

## Example lifecycle

```text
producer P1 creates batch B-2026-044
lab L2 attaches certification hash sha256:9a…
P1 initiates transfer to shipper S4
S4 accepts transfer
retailer R7 accepts final transfer
auditor reads the ordered custody history
```

## Security and governance

- Do not put personal, secret, or legally erasable documents on-chain.
- Use domain-separated canonical hashing and define byte serialization exactly.
- Protect against replay, duplicate IDs, unauthorized roles, stale transfer acceptance, and arithmetic overflow.
- Define key loss, organization suspension, contract upgrade, and emergency pause governance.
- Require independent review and platform-specific audit before real asset or financial use.

## Required tests

### Contract unit tests

- `T01` — Only an authorized producer can create a batch.
- `T02` — Only the intended recipient can accept a pending transfer.
- `T03` — Replayed or stale transfer calls change no state.
- `T04` — Recalled and closed batches reject further custody transitions.
- `T05` — Certification expiry and issuer authorization follow policy.

### Integration tests

- `T06` — Deploy locally and complete the documented multi-party lifecycle.
- `T07` — Every state mutation emits exactly one versioned event.
- `T08` — The indexer rebuilds materialized state from genesis events.
- `T09` — Reorganization or finality simulation produces no duplicate finalized event.
- `T10` — Document verification accepts matching bytes and rejects altered bytes.

### Adversarial and operational tests

- `T11` — Property-test state-machine invariants over generated call sequences.
- `T12` — Fuzz call decoding and canonical serialization.
- `T13` — Measure worst-case execution cost and enforce transaction limits.
- `T14` — Upgrade a local deployment while preserving storage and event compatibility.

## Local deployment and production design exercise

Run the contract/runtime, indexer, reorganization/finality simulation, and replay tools on a local
network. A public test network is optional; never require fees, credentials, or a production chain
to complete the learning evidence.

Demonstrate durable checkpoints, database backup/restore, lag alerts, RPC failover simulation, replay,
artifact pinning, and reproducible builds. Treat production governance/network choice as a design
review requiring independent audit, not as a deployment claim.

## Acceptance criteria

Independent organizations can complete the lifecycle with authorized signatures, auditors can reconstruct ordered provenance, documents remain off-chain, and contract plus indexer state survive restart and replay.

## System-design practice

Declare organizations, batches, events/batch, document sizes, chain finality, query mix, and
execution/storage cost limits. Measure submission/finality/indexing p95/p99, reorganization lag,
throughput, and cost/event. Senior proves authorized provenance; Staff adds indexer replay,
governance/version migration, participant fairness, privacy deletion boundaries, and failover;
Principal compares blockchain with a consortium database, defines ownership and dispute authority,
regional/legal policy, long-term cost, upgrade, and exit migration. A design review must justify why
multiple writers without one administrator require this trust model.

## Optional extensions

Add delegated roles, batch splitting and merging, QR verification, selective disclosure, or cross-chain anchoring.
