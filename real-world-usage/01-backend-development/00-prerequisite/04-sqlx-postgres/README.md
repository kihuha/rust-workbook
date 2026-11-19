# 04 — PostgreSQL with SQLx

## Goal

Replace the in-memory task store with PostgreSQL and SQLx. Build a repository whose correctness depends on database constraints, explicit transaction boundaries, and understood locking—not on happy-path CRUD.

Use the [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/), [`query!` macro](https://docs.rs/sqlx/latest/sqlx/macro.query.html), [SQLx migrations](https://docs.rs/sqlx/latest/sqlx/migrate/), and PostgreSQL documentation for [transactions](https://www.postgresql.org/docs/current/tutorial-transactions.html), [isolation](https://www.postgresql.org/docs/current/transaction-iso.html), and [locking](https://www.postgresql.org/docs/current/explicit-locking.html).

## Schema and operations

Create `tenants`, `users`, `tasks`, `task_events`, and `idempotency_records`. Include foreign keys, unique constraints, checks, timestamps, and an integer version. Implement create, tenant-scoped lookup, stable cursor listing, optimistic update, archive, and append-only event creation.

## Required database work

- Manage an explicitly sized connection pool with acquire/connect timeouts.
- Write forward migrations and a documented rollback/forward-fix policy.
- Use compile-checked SQL or checked offline metadata in CI.
- Map database rows into domain types without leaking SQLx types through the domain API.
- Create a task, event, and idempotency result in one transaction.
- Demonstrate `READ COMMITTED` behavior, one lost-update hazard, and the chosen defense.
- Use `SELECT ... FOR UPDATE` for one workflow and consistent lock ordering for multi-row work.
- Translate unique, foreign-key, check, timeout, and serialization failures into typed repository errors.
- Explain why application checks cannot replace constraints under concurrency.

## Required tests

- `T01` — Migrations build an empty disposable database from zero.
- `T02` — Every repository operation enforces `tenant_id` in SQL.
- `T03` — A create transaction rolls back task, event, and idempotency rows together on injected failure.
- `T04` — Two concurrent updates cannot silently lose one committed change.
- `T05` — Duplicate idempotency keys return the original result or a conflict according to request hash.
- `T06` — Stable pagination neither repeats nor skips equal-timestamp rows.
- `T07` — Pool exhaustion reaches a bounded timeout instead of waiting forever.
- `T08` — Opposite lock order reproduces a deadlock; corrected order makes the test reliable.
- `T09` — Backup/restore or dump/restore reproduces schema and representative rows.

## Completion evidence

Provide an entity diagram, transaction-boundary notes, query-plan output for the list endpoint, pool-sizing rationale, migration procedure, and repository integration tests against real PostgreSQL. You should be able to choose constraints, optimistic versions, row locks, or serializable retry based on the invariant.
