# 06 — Authentication and authorization

## Goal

Add three realistic identity modes to a small Axum service: browser sessions, short-lived bearer JWTs, and opaque machine API keys. Enforce scopes and tenant ownership below the handler layer.

Use [`argon2`](https://docs.rs/argon2/latest/argon2/), [`jsonwebtoken`](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/), [`tower-sessions`](https://docs.rs/tower-sessions/latest/tower_sessions/), [`axum-login`](https://docs.rs/axum-login/latest/axum_login/), [`secrecy`](https://docs.rs/secrecy/latest/secrecy/), and the [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html) as the security reference.

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
