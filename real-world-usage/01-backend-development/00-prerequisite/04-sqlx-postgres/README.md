# 04 — PostgreSQL with SQLx

## Goal

Replace the in-memory task store with PostgreSQL and SQLx. Build a repository whose correctness depends on database constraints, explicit transaction boundaries, and understood locking—not on happy-path CRUD.

Use the [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/), [`query!` macro](https://docs.rs/sqlx/latest/sqlx/macro.query.html), [SQLx migrations](https://docs.rs/sqlx/latest/sqlx/migrate/), and PostgreSQL documentation for [transactions](https://www.postgresql.org/docs/current/tutorial-transactions.html), [isolation](https://www.postgresql.org/docs/current/transaction-iso.html), and [locking](https://www.postgresql.org/docs/current/explicit-locking.html).

## Before you start

Use the domain and application service from challenge 02; middleware from challenge 03 may remain, but it is not the focus. You should know tables, rows, primary keys, foreign keys, and basic `SELECT`/`INSERT`/`UPDATE`. If joins and transactions are entirely new, first work through the PostgreSQL tutorial linked above in `psql`.

Start a disposable local PostgreSQL with Docker Compose. Put only non-secret development defaults in Compose; keep `DATABASE_URL` in an ignored `.env` or your shell. Add a health check so tests do not race database startup. Install `sqlx-cli` using the installation command in the SQLx version's documentation, then verify you can create a database and run an empty migration before changing application code.

## Mental model

The database is a concurrent component, not a file behind your repository. A transaction makes several statements commit or roll back together; it does not automatically prevent every race. Constraints protect invariants even when two application instances act at once. The pool is a bounded queue of database sessions, so its size and acquisition timeout are part of service capacity.

## Suggested schema

- `tenants(id, name, created_at)`
- `users(id, tenant_id, display_name, created_at)`
- `tasks(id, tenant_id, owner_id, title, description, status, version, archived_at, created_at, updated_at)`
- `task_events(id, tenant_id, task_id, kind, payload, created_at)`
- `idempotency_records(tenant_id, key, request_hash, response_status, response_body, created_at)`

Write down each key, check, unique constraint, and delete behavior before writing the migration. A `tenant_id` column on tenant-owned tables is deliberate redundancy that makes safe query scoping visible.

## Build it in milestones

### 1. Migrate an empty database

Create the tables and constraints in small forward migrations. Add a development seed command separately; migrations must not depend on seed data. Run from an empty database twice by recreating the database, not by assuming a developer's current state.

**Checkpoint:** one command brings a fresh database to the latest schema; invalid status, duplicate idempotency key, and missing parent rows are rejected by PostgreSQL.

### 2. Learn SQLx with one operation

Configure a small pool with connect and acquire timeouts. Implement tenant-scoped `get_task` using checked SQL. Map a private row type into the existing domain type. Do not expose `PgPool`, `sqlx::Error`, or database row types from the application-facing repository API.

**Checkpoint:** a real-database test inserts a fixture and reads it for the correct tenant; the same ID under another tenant returns not found.

### 3. Replace the in-memory repository

Implement create, list, update, and archive one at a time. Include `tenant_id` in every tenant-owned query, including updates. Use `(created_at, id)` for stable cursor ordering and version in the update predicate for optimistic concurrency.

**Checkpoint:** run the existing application/API contract tests once against memory and once against PostgreSQL; stale concurrent updates cannot both succeed.

### 4. Make one workflow atomic

Create a task, its first event, and its idempotency record in one explicit transaction. Inject an error between statements in a test and let the transaction roll back. Return the original stored result when the same key and request hash are repeated; return conflict for a different hash.

**Checkpoint:** after injected failure, none of the three rows exists. A repeated equal request creates no new task or event.

### 5. Observe locking and pool pressure

Write a focused experiment with two transactions. Reproduce a lost-update attempt, then defend it with the version predicate. Use `FOR UPDATE` for a separate state-transition workflow. Finally, hold every pool connection and prove another acquisition times out.

**Checkpoint:** tests make each concurrency outcome visible and finish under a fixed deadline.

### 6. Inspect and operate it

Run `EXPLAIN (ANALYZE, BUFFERS)` for the list query with representative data, add only an index justified by the plan, and repeat. Dump, recreate, and restore a small database.

**Checkpoint:** the README records the plan change, pool rationale, migration command, and tested restore command.

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

These are graduation tests after the milestone checkpoints pass. Run them serially or isolate their databases when they mutate shared schema/state.

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
