# 00 — Backend framework prerequisites

This track bridges Rust language knowledge and realistic backend projects. Each challenge isolates one production concern before the integration capstone combines them. Complete the required challenges in order before starting PayForge or BankBridge.

SwiftCart is intentionally excluded from this stack: it uses standard-library TCP, manual HTTP parsing, and bounded threads to expose what frameworks automate.

## Required stack

The shared baseline is Tokio, Axum, Tower, PostgreSQL, SQLx, Serde, Reqwest, `tracing`, OpenTelemetry, and Prometheus-compatible metrics. SeaORM is taught after direct SQL. Authentication and rate limiting use focused libraries on top of Axum/Tower.

Pin compatible crate versions in `Cargo.lock`. Record the minimum supported Rust version and do not assume examples written against `latest` documentation work with older pinned releases.

## Challenge sequence

| No. | Challenge | Delivery |
| --- | --- | --- |
| 01 | [Tokio runtime and cancellation](01-tokio-runtime/README.md) | Bounded concurrent job service with deterministic shutdown |
| 02 | [Axum API fundamentals](02-axum-api/README.md) | Typed task API with stable errors and in-process tests |
| 03 | [Tower middleware and traffic control](03-tower-middleware/README.md) | Ordered middleware stack with quotas and overload protection |
| 04 | [PostgreSQL with SQLx](04-sqlx-postgres/README.md) | Transactional repository with migrations and locking tests |
| 05 | [SeaORM data modelling](05-seaorm-data-modeling/README.md) | Relational read model plus SQLx/ORM comparison |
| 06 | [Authentication and authorization](06-authentication-authorization/README.md) | Sessions, JWTs, opaque credentials, scopes, and tenant isolation |
| 07 | [Outbound integrations and reliability](07-outbound-reliability/README.md) | Failure-aware connector with safe retries and idempotency |
| 08 | [Observability and operations](08-observability-operations/README.md) | Traces, metrics, health, configuration, and shutdown under load |
| 09 | [Production integration capstone](09-integration-capstone/README.md) | Multi-tenant service combining the complete stack |

## Working standard

Every challenge must include unit and integration tests, bounded resources, explicit timeouts, graceful cancellation, typed errors, and documentation explaining the chosen failure behavior. Run infrastructure-dependent tests against real disposable services rather than mocks when the behavior belongs to PostgreSQL or the network.

The primary documentation map for this track is [Rust backend framework documentation](../../../research/rust-backend-frameworks.md).

## Exit criteria

You are ready for PayForge or BankBridge when you can explain middleware order, trace an authenticated request through a database transaction and outbound call, prevent cross-tenant access in the query layer, apply keyed quotas, diagnose failure from telemetry, and shut down without losing acknowledged work.
