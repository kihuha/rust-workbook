# 05 — SeaORM data modelling

## Goal

Build `support-read-model`, a relational customer-support view using SeaORM. Learn what an async ORM improves, what SQL it emits, and when explicit SQLx remains the clearer correctness tool.

Use the [SeaORM documentation](https://www.sea-ql.org/SeaORM/docs/index/), [SeaORM crate documentation](https://docs.rs/sea-orm/latest/sea_orm/), and [Diesel documentation](https://docs.diesel.rs/master/diesel/index.html) for the design comparison.

## Data model

Model organizations, users, support tickets, labels, ticket-label joins, comments, and assignees. Support required and optional relations, composite uniqueness, soft archive, and paginated ticket summaries with comment counts and labels.

## Required ORM work

- Define entities, primary keys, relations, active models, and migrations.
- Create an organization, initial user, and default label in one transaction.
- Load a ticket detail without an N+1 query pattern.
- Produce a filtered, stable, paginated ticket summary.
- Update allowed fields with an optimistic version check.
- Inspect and record generated SQL for the detail and list paths.
- Reimplement one complex summary query with SQLx and compare clarity, query shape, type mapping, and transaction control.
- Write a short comparison with Diesel's schema/query-builder approach; no Diesel implementation is required.

## Constraints

Keep SeaORM entities in the persistence layer. Domain types must not derive ORM traits. Do not hide authorization in a late in-memory filter. Raw SQL escape hatches require a comment naming the invariant or query-shape reason.

## Required tests

- `T01` — Entity relations load the correct graph for multiple tickets and labels.
- `T02` — The list endpoint executes a bounded number of queries independent of row count.
- `T03` — A failed multi-entity create leaves no partial rows.
- `T04` — Concurrent stale updates cannot overwrite the newest ticket version.
- `T05` — Organization scoping is present in entity and raw-query paths.
- `T06` — Pagination remains stable across joins and equal sort keys.
- `T07` — SeaORM and SQLx implementations produce the same summary values.

## Completion evidence

Submit the entity graph, captured SQL, query-count evidence, and a one-page decision record: “When this codebase should use SeaORM, SQLx, or a database view.” You should finish able to use an ORM without mistaking it for a transaction or authorization model.
