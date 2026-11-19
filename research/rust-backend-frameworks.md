# Rust backend framework documentation map

Research date: 2026-11-19

## Scope

This note maps primary documentation for the framework stack needed after a bare-bones HTTP server and before the PayForge and BankBridge projects. It favors the libraries' own documentation, project sites, repositories, and the PostgreSQL manual. The intended baseline is Tokio + Axum + Tower + PostgreSQL, with SQLx taught before an ORM.

Documentation links below use each project's current documentation rather than freezing a crate version. Project starter workspaces should pin compatible versions in `Cargo.lock` and record the minimum supported Rust version because the `latest` documentation can advance independently.

## Primary-source matrix

| Concern | Primary documentation | What it establishes for the curriculum |
| --- | --- | --- |
| Async runtime | [Tokio tutorial](https://tokio.rs/tokio/tutorial), [Tokio crate docs](https://docs.rs/tokio/latest/tokio/) | Runtime, spawning, shared state, channels, async I/O, framing, and `select!` |
| Shutdown | [Tokio graceful shutdown topic](https://tokio.rs/tokio/topics/shutdown) | Detect shutdown, notify tasks, and wait for completion |
| HTTP framework | [Axum crate docs](https://docs.rs/axum/latest/axum/) | Routing, handlers, extractors, responses, state, errors, middleware, and serving |
| WebSockets | [Axum WebSocket extractor](https://docs.rs/axum/latest/axum/extract/ws/index.html) | Upgrade extraction and split send/receive work; optional for these projects |
| Service/middleware model | [Tower crate docs](https://docs.rs/tower/latest/tower/), [`ServiceBuilder`](https://docs.rs/tower/latest/tower/struct.ServiceBuilder.html) | `Service`, `Layer`, middleware composition, and order-sensitive layering |
| SQL access | [SQLx crate docs](https://docs.rs/sqlx/latest/sqlx/), [`query!`](https://docs.rs/sqlx/latest/sqlx/macro.query.html), [migrations](https://docs.rs/sqlx/latest/sqlx/migrate/index.html) | Async pools, transactions, checked SQL, migrations, and supported databases |
| ORM | [SeaORM documentation](https://www.sea-ql.org/SeaORM/docs/index/), [SeaORM crate docs](https://docs.rs/sea-orm/latest/sea_orm/) | Async ORM entities, relations, queries, transactions, migrations, and SQLx foundation |
| ORM comparison | [Diesel documentation](https://docs.diesel.rs/master/diesel/index.html) | Compile-time query builder and schema-oriented ORM alternative |
| Database semantics | [PostgreSQL transactions](https://www.postgresql.org/docs/current/tutorial-transactions.html), [transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html), [explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html) | Atomicity, savepoints, isolation anomalies, row/table locks, and deadlocks |
| Serialization | [Serde documentation](https://serde.rs/) | Derive-based serialization/deserialization and data-format separation |
| Input validation | [`validator` crate docs](https://docs.rs/validator/latest/validator/) | Derive-based validation, nested validation, and custom validators |
| Password hashing | [`argon2` crate docs](https://docs.rs/argon2/latest/argon2/) | Argon2 password hashing and PHC string parsing/verification |
| JWT | [`jsonwebtoken` crate docs](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/) | Signed token encoding/decoding, algorithms, headers, keys, and claim validation |
| Server sessions | [`tower-sessions` crate docs](https://docs.rs/tower-sessions/latest/tower_sessions/) | Tower/Axum session middleware, pluggable stores, expiry, and signed/private cookies via features |
| Authentication integration | [`axum-login` crate docs](https://docs.rs/axum-login/latest/axum_login/) | Axum/Tower session-backed authentication and authorization building blocks |
| Rate limiting | [Tower `RateLimitLayer`](https://docs.rs/tower/latest/tower/limit/rate/struct.RateLimitLayer.html), [`tower-governor` docs](https://docs.rs/tower_governor/latest/tower_governor/), [`governor` docs](https://docs.rs/governor/latest/governor/) | Service-wide limits and key-based quota algorithms/middleware |
| Structured diagnostics | [`tracing` docs](https://docs.rs/tracing/latest/tracing/), [`tracing-subscriber` docs](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/) | Spans/events plus configurable collection, filtering, and formatting |
| Distributed telemetry | [OpenTelemetry Rust docs](https://opentelemetry.io/docs/languages/rust/), [`tracing-opentelemetry` docs](https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/) | Rust telemetry status and bridging tracing spans to OpenTelemetry |
| Metrics | [`metrics` docs](https://docs.rs/metrics/latest/metrics/), [`metrics-exporter-prometheus` docs](https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/) | Recorder/facade model and Prometheus exporter/HTTP rendering |
| Configuration | [`config` crate docs](https://docs.rs/config/latest/config/), [`secrecy` crate docs](https://docs.rs/secrecy/latest/secrecy/) | Layered configuration sources and intentional secret exposure/redacted debug output |
| In-process HTTP tests | [Tower `ServiceExt`](https://docs.rs/tower/latest/tower/util/trait.ServiceExt.html), [Axum testing example](https://github.com/tokio-rs/axum/tree/main/examples/testing) | `oneshot` service tests without a listening socket |
| External HTTP tests | [`reqwest` docs](https://docs.rs/reqwest/latest/reqwest/), [`wiremock` docs](https://docs.rs/wiremock/latest/wiremock/) | Async HTTP client and mock HTTP server for adapter tests |

## Runtime and HTTP framework findings

Tokio's tutorial deliberately covers spawning tasks, shared state, channels, I/O, framing, and `select!`; its crate documentation describes Tokio as an event-driven, non-blocking platform with runtime, asynchronous APIs, and synchronization primitives. Prerequisites therefore need more than `async fn`: learners must understand task ownership, `Send`, cancellation, bounded channels, avoiding blocking work on runtime threads, and timeout behavior ([Tokio tutorial](https://tokio.rs/tokio/tutorial), [Tokio crate docs](https://docs.rs/tokio/latest/tokio/)).

Tokio's shutdown guidance splits graceful shutdown into detecting the signal, telling every task to stop, and waiting for task termination. A prerequisite should make learners track spawned work and prove in-flight requests/jobs drain under a deadline ([Tokio graceful shutdown](https://tokio.rs/tokio/topics/shutdown)).

Axum builds applications from routes and handlers, uses extractors for request data and `IntoResponse` for responses, shares state through `State`, and uses Tower middleware. Its error model expects errors returned by handlers/services to become responses so the serving stack remains infallible at the protocol boundary ([Axum crate docs](https://docs.rs/axum/latest/axum/), [Axum error handling](https://docs.rs/axum/latest/axum/error_handling/index.html)).

Axum's WebSocket module supports HTTP upgrade and exposes a socket that may be split into sending and receiving halves. That is useful as an optional streaming exercise, but neither payment nor financial aggregation requires it as a prerequisite ([Axum WebSocket extractor](https://docs.rs/axum/latest/axum/extract/ws/index.html)).

## Tower middleware findings

Tower's core abstraction is an asynchronous `Service<Request>` and its composable `Layer` middleware. `ServiceBuilder` documents that layers are applied in the order they are added, so order changes what each layer observes; middleware order must be tested rather than treated as cosmetic ([Tower crate docs](https://docs.rs/tower/latest/tower/), [`ServiceBuilder`](https://docs.rs/tower/latest/tower/struct.ServiceBuilder.html)).

Tower supplies timeout, concurrency-limit, load-shed, buffer, retry, and rate-limit modules. The curriculum should distinguish overload controls: concurrency bounds in-flight work, load shedding rejects when a service is not ready, timeout bounds waiting, and buffering moves backpressure into a bounded queue ([Tower limit module](https://docs.rs/tower/latest/tower/limit/index.html), [Tower load-shed module](https://docs.rs/tower/latest/tower/load_shed/index.html), [Tower timeout module](https://docs.rs/tower/latest/tower/timeout/index.html), [Tower buffer module](https://docs.rs/tower/latest/tower/buffer/index.html)).

Tower's built-in `RateLimitLayer` limits a service as a whole. `tower-governor` integrates the `governor` algorithm with Tower/Axum and provides key extractors, making it the more suitable teaching tool for per-IP or authenticated-principal quotas; production exercises must define trusted proxy handling and the key on which a quota is enforced ([Tower `RateLimitLayer`](https://docs.rs/tower/latest/tower/limit/rate/struct.RateLimitLayer.html), [`tower-governor`](https://docs.rs/tower_governor/latest/tower_governor/), [`governor`](https://docs.rs/governor/latest/governor/)).

## Persistence and ORM findings

SQLx provides asynchronous, pure-Rust SQL access with connection pools, transactions, and drivers for PostgreSQL and other databases. Its `query!` family checks SQL against a database at build time or against prepared offline metadata, while its migration module can embed and execute migrations ([SQLx crate docs](https://docs.rs/sqlx/latest/sqlx/), [`query!`](https://docs.rs/sqlx/latest/sqlx/macro.query.html), [SQLx migrations](https://docs.rs/sqlx/latest/sqlx/migrate/index.html)).

Learners should use SQLx directly before an ORM so transaction boundaries, query plans, constraints, row locking, and domain-to-row mapping remain visible. PostgreSQL defines transactions as all-or-nothing groups, supports savepoints, documents isolation-level anomalies, and warns that explicit locking can create deadlocks whose best defense is consistent lock ordering ([PostgreSQL transactions](https://www.postgresql.org/docs/current/tutorial-transactions.html), [transaction isolation](https://www.postgresql.org/docs/current/transaction-iso.html), [explicit locking](https://www.postgresql.org/docs/current/explicit-locking.html)).

SeaORM is the recommended ORM challenge after SQLx because its official documentation presents it as async, built on SQLx, and organized around entities, relations, query builders, transactions, and migrations. This matches a Tokio/Axum course without hiding the previously learned database concepts ([SeaORM documentation](https://www.sea-ql.org/SeaORM/docs/index/), [SeaORM crate docs](https://docs.rs/sea-orm/latest/sea_orm/)).

Diesel remains a valuable comparison: its documentation emphasizes a strongly typed query builder and schema-safe queries. It should appear in a design-comparison exercise rather than becoming a second required implementation stack, avoiding two ORM APIs before the capstones ([Diesel documentation](https://docs.diesel.rs/master/diesel/index.html)).

## Authentication, validation, and secrets findings

Serde separates Rust data structures from data formats through `Serialize` and `Deserialize`; `validator` adds derive-based field, nested, and custom validation. Learners must keep deserialization, syntactic validation, authorization, and domain invariants as separate layers ([Serde documentation](https://serde.rs/), [`validator` docs](https://docs.rs/validator/latest/validator/)).

The RustCrypto `argon2` crate implements Argon2 password hashing and password-hash verification through PHC strings. The `jsonwebtoken` crate encodes and decodes signed JWTs with explicit algorithms, keys, headers, and validation settings. Challenges should require password verification without logging secrets and token validation that fixes the accepted algorithm, issuer, audience, and expiry rather than merely decoding claims ([`argon2` docs](https://docs.rs/argon2/latest/argon2/), [`jsonwebtoken` docs](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/)).

`tower-sessions` provides Tower/Axum session middleware and pluggable stores, and `axum-login` builds authentication/authorization around such sessions. Teaching both cookie-backed sessions and bearer JWTs makes the learner choose based on client and revocation requirements instead of treating JWT as the default ([`tower-sessions` docs](https://docs.rs/tower-sessions/latest/tower_sessions/), [`axum-login` docs](https://docs.rs/axum-login/latest/axum_login/)).

The `config` crate layers configuration sources and environment variables; `secrecy` wraps secret values so accidental debug output is redacted and plaintext access is explicit. These are application safeguards, not a substitute for an external production secret manager ([`config` docs](https://docs.rs/config/latest/config/), [`secrecy` docs](https://docs.rs/secrecy/latest/secrecy/)).

## Observability and test findings

`tracing` models structured, contextual events and spans; `tracing-subscriber` collects them with filters, formatters, and composable layers. A prerequisite should instrument one request across handler, database, and outbound HTTP work while proving credentials and personal data do not enter fields ([`tracing` docs](https://docs.rs/tracing/latest/tracing/), [`tracing-subscriber` docs](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)).

OpenTelemetry Rust documents traces, metrics, and logs and currently marks language-signal maturity separately; `tracing-opentelemetry` connects tracing spans to an OpenTelemetry tracer. Curriculum requirements should pin versions and treat exporters as replaceable infrastructure because ecosystem maturity and compatibility change ([OpenTelemetry Rust](https://opentelemetry.io/docs/languages/rust/), [`tracing-opentelemetry`](https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/)).

The `metrics` crate is a facade driven by an installed recorder, while `metrics-exporter-prometheus` supplies a recorder and handle for rendering Prometheus data. Learners should define low-cardinality counters, histograms, and gauges and test that user/object IDs never become metric labels ([`metrics` docs](https://docs.rs/metrics/latest/metrics/), [Prometheus exporter docs](https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/)).

Axum routers are Tower services, so `ServiceExt::oneshot` can test requests in process; the Axum repository maintains a testing example. `reqwest` supplies an async client for tests against a bound server, and `wiremock` supplies a mock HTTP server for connector/processor behavior. The prerequisite sequence should include unit, repository, in-process HTTP, real-server, and failing-upstream tests ([Tower `ServiceExt`](https://docs.rs/tower/latest/tower/util/trait.ServiceExt.html), [Axum testing example](https://github.com/tokio-rs/axum/tree/main/examples/testing), [`reqwest`](https://docs.rs/reqwest/latest/reqwest/), [`wiremock`](https://docs.rs/wiremock/latest/wiremock/)).

## Curriculum implications

The prerequisite directory should be a sequence of independently reviewable challenges rather than one large tutorial:

1. **Tokio runtime and cancellation:** tasks, `Send`, bounded channels, timeout, cancellation, blocking boundaries, and graceful shutdown, using a small concurrent job service ([Tokio tutorial](https://tokio.rs/tokio/tutorial), [shutdown topic](https://tokio.rs/tokio/topics/shutdown)).
2. **Axum API fundamentals:** routes, typed extractors, state, responses, error envelopes, JSON limits, and `oneshot` tests ([Axum crate docs](https://docs.rs/axum/latest/axum/), [testing example](https://github.com/tokio-rs/axum/tree/main/examples/testing)).
3. **Tower production middleware:** request IDs/traces, auth, body limits, timeout, concurrency, load shedding, rate limiting, and deliberately tested layer order ([Tower crate docs](https://docs.rs/tower/latest/tower/), [`ServiceBuilder`](https://docs.rs/tower/latest/tower/struct.ServiceBuilder.html)).
4. **PostgreSQL with SQLx:** migrations, pools, compile-checked queries, transactions, constraints, isolation, row locks, and repository tests against a real database ([SQLx docs](https://docs.rs/sqlx/latest/sqlx/), [PostgreSQL transactions](https://www.postgresql.org/docs/current/tutorial-transactions.html)).
5. **SeaORM and data modelling:** rebuild a bounded CRUD/read-model slice with entities and relations, then compare the generated SQL and transaction control with SQLx; discuss Diesel as a typed alternative ([SeaORM docs](https://www.sea-ql.org/SeaORM/docs/index/), [Diesel docs](https://docs.diesel.rs/master/diesel/index.html)).
6. **Authentication and authorization:** Argon2 credentials, session and JWT alternatives, fixed validation policy, role/scope checks, tenant-safe queries, rotation, and revocation ([`argon2`](https://docs.rs/argon2/latest/argon2/), [`jsonwebtoken`](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/), [`tower-sessions`](https://docs.rs/tower-sessions/latest/tower_sessions/)).
7. **Outbound integrations and reliability:** Reqwest timeouts, connector traits, Wiremock failures, retries only for safe operations, circuit breaking, and idempotency ([`reqwest`](https://docs.rs/reqwest/latest/reqwest/), [`wiremock`](https://docs.rs/wiremock/latest/wiremock/), [Tower retry module](https://docs.rs/tower/latest/tower/retry/index.html)).
8. **Observability and operations:** redacted tracing, OpenTelemetry propagation, low-cardinality Prometheus metrics, health/readiness, configuration/secrets, and shutdown under load ([`tracing`](https://docs.rs/tracing/latest/tracing/), [OpenTelemetry Rust](https://opentelemetry.io/docs/languages/rust/), [Prometheus exporter](https://docs.rs/metrics-exporter-prometheus/latest/metrics_exporter_prometheus/)).
9. **Integration capstone:** combine the stack in a small multi-tenant service with migrations, authentication, quotas, an outbox worker, an unreliable upstream, and a full test pyramid before attempting PayForge or BankBridge (sources above).

SwiftCart should remain the contrast case: it manually handles TCP/HTTP and should not silently import Axum, Tokio, SQLx, or an ORM. Its purpose is to expose protocol parsing, connection lifecycle, bounded concurrency, and graceful shutdown before frameworks automate those seams. PayForge and BankBridge should state an explicit shared stack based on the sequence above, while allowing domain-specific additions.

## Recommended stack boundary

Use **Tokio + Axum + Tower + PostgreSQL + SQLx** as the required shared production stack. Teach **SeaORM** in one focused prerequisite so learners understand an ORM, but do not require it in both capstones: PayForge benefits from explicit SQL and transaction/locking control, while BankBridge can use SQLx repositories or SeaORM for ordinary entity relationships as long as sync transactions remain explicit. Use **tracing**, **OpenTelemetry**, and **metrics/Prometheus** for observability; **Argon2**, **tower-sessions/axum-login**, and **jsonwebtoken** as contrasting authentication tools; and **tower-governor** for keyed rate limiting.

This boundary keeps the framework list finite while covering the operational concerns the capstones actually require.
