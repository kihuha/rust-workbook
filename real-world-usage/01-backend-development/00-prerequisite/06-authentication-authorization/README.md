# 06 — Authentication and authorization

## Goal

Add three realistic identity modes to a small Axum service: browser sessions, short-lived bearer JWTs, and opaque machine API keys. Enforce scopes and tenant ownership below the handler layer.

Use [`argon2`](https://docs.rs/argon2/latest/argon2/), [`jsonwebtoken`](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/), [`tower-sessions`](https://docs.rs/tower-sessions/latest/tower_sessions/), [`axum-login`](https://docs.rs/axum-login/latest/axum_login/), [`secrecy`](https://docs.rs/secrecy/latest/secrecy/), and the [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html) as the security reference.

## Before you start

Complete challenges 02–04 and the quota portion of challenge 03. Your database queries must already require a tenant ID. Create a small `identity-lab` service or branch `taskboard-api`; do not learn three credential types inside a payment project.

Use test keys and local cookies only. This exercise teaches application design, not how to operate a public identity provider. Do not invent cryptographic algorithms, password reset, email verification, social login, or multi-factor authentication here.

## Mental model

Authentication answers “which principal presented this request?” Authorization answers “may that principal perform this action on this tenant/object?” A session, JWT, and API key are credential transport/lifecycle choices, not authorization systems. All successful modes become one typed `Principal`; use cases and repositories accept that trusted identity and still scope their queries.

Begin with an authorization matrix:

| Actor | Credential | Allowed example | Denied example |
| --- | --- | --- | --- |
| Operator | browser session | view own tenant's tasks | manage another tenant |
| Service client | scoped JWT | read tasks | create without `tasks:write` |
| Tenant integration | API key | create task | administer users |

Expand this table before writing route guards.

## Build it in milestones

### 1. Password storage, not login

Write a small `PasswordService` that hashes a bounded password with Argon2 using a fresh salt and verifies a PHC string. Run it in `spawn_blocking` behind a semaphore because hashing is CPU/memory intensive. Wrap password inputs in secret types and use generic public failure messages.

**Checkpoint:** correct/wrong passwords behave distinctly inside the service, while the public error does not reveal whether a user exists; logs contain a known canary in neither path.

### 2. One server-side browser session

Implement login, current-user, and logout with a server-side session store. Rotate the session ID after login, set secure cookie attributes appropriate to local/production modes, and distinguish idle from absolute expiry. Add CSRF protection to cookie-authenticated mutations before adding more routes.

**Checkpoint:** login changes the session identifier; missing/mismatched CSRF tokens fail; logout and simulated expiry invalidate access.

### 3. One short-lived JWT

Issue a token only from a test/admin path. Validate a fixed algorithm, signature, issuer, audience, `exp`, `nbf`, subject, tenant, scopes, and key ID. Keep encoding/decoding types at the identity boundary and map claims into `Principal` only after all validation passes.

**Checkpoint:** start with one valid token, mutate one property at a time, and prove every mutation is rejected for the intended reason.

### 4. One opaque API key

Generate a high-entropy random secret with a public key ID prefix. Display the full value once; store a keyed hash/verification value and metadata, never the credential. Implement last-used metadata, rotation overlap, and immediate revocation.

**Checkpoint:** the issued value authenticates, database/log dumps cannot recover it, and revocation fails on the next request.

### 5. Unify authorization and tenant isolation

Write one extractor/middleware that yields `Principal`. Put scope/role decisions in application policies and object/tenant constraints in repository functions. Do not accept a tenant ID from a request and trust it merely because it matches a path.

**Checkpoint:** run a negative matrix that changes path, body, query, cursor, and claims independently; no variant reads or changes the other tenant.

### 6. Add lifecycle and audit behavior

Rate-limit login/API-key failures, define CORS explicitly, record redacted audit events, and document key/session/token rotation and emergency revocation. Check that error timing/content does not become an obvious account-enumeration API.

**Checkpoint:** a reviewer can follow the authorization matrix and rotation runbook without reading implementation code.

## Required flows

- Register and verify a password stored as an Argon2 PHC string.
- Log in through a secure server-side session, rotate the session identifier, enforce idle/absolute expiry, and log out.
- Issue a short-lived JWT for a named audience with subject, tenant, scopes, issued-at, not-before, expiry, and key ID.
- Validate a fixed algorithm, issuer, audience, time claims, key state, and required scopes before accepting claims.
- Create a high-entropy opaque API key, display it once, store only a keyed lookup hash plus metadata, and support rotation/revocation.
- Attach one typed `Principal` enum to requests so handlers do not parse credentials.
- Enforce tenant and object ownership inside application/repository calls.
- Record redacted audit events for login, failures, key creation, rotation, revocation, and denied privileged operations.

## Security constraints

Use generic login failures, constant-time verification where supported, bounded credential input, rate limiting, secure cookie attributes, CSRF protection for cookie-authenticated mutations, and an explicit CORS policy. Never log passwords, hashes, cookies, JWTs, API keys, reset tokens, or full authorization headers.

Document why sessions, JWTs, and API keys are each used. Do not use JWTs where immediate revocation is a hard requirement without an additional revocation design.

## Required tests

These are graduation tests after the milestone checkpoints pass. Keep secret-bearing fixtures obviously fake and scan captured output for them.

- `T01` — Correct passwords verify and wrong passwords fail without exposing which check failed.
- `T02` — Login rotates the session ID; logout and expiry invalidate it.
- `T03` — CSRF-protected session mutations reject missing or mismatched tokens.
- `T04` — JWT validation rejects wrong algorithm, signature, issuer, audience, time window, key ID, and scope.
- `T05` — An API key is shown once; database/log inspection cannot recover it.
- `T06` — Rotated credentials overlap only for the documented window; revoked credentials fail immediately.
- `T07` — Tenant A cannot access Tenant B by changing path, query, body, cursor, or token claims.
- `T08` — Authentication and quota errors remain stable without becoming an account-enumeration oracle.

## Completion evidence

Provide threat-model diagrams, credential lifecycles, authorization matrix, rotation runbook, and negative test suite. Explain authentication versus authorization, browser versus machine threats, and why tenant-safe repository APIs are stronger than handler checks alone.
