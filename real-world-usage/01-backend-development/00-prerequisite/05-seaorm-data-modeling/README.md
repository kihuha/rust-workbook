# 05 — SeaORM data modelling

## Goal

Build `support-read-model`, a relational customer-support view using SeaORM. Learn what an async ORM improves, what SQL it emits, and when explicit SQLx remains the clearer correctness tool.

Use the [SeaORM documentation](https://www.sea-ql.org/SeaORM/docs/index/), [SeaORM crate documentation](https://docs.rs/sea-orm/latest/sea_orm/), and [Diesel documentation](https://docs.diesel.rs/master/diesel/index.html) for the design comparison.

## Before you start

Complete challenge 04, including one transaction and stable pagination. This challenge assumes you can read the SQL that an ORM emits; it does not teach relational databases from scratch. Create a separate `support-read-model` package and reuse the disposable PostgreSQL setup, not the task domain code.

This is a comparison exercise, not an instruction to replace SQLx everywhere. Stop and inspect generated SQL whenever an ORM method feels magical.

## What you are building

An operator needs a list of support tickets showing subject, state, assignee, labels, comment count, and last activity, plus a detailed ticket view. Use a small fixture with two organizations, several users, tickets with and without assignees, shared labels, and multiple comments. Every query begins from an organization identity.

## Mental model

An ORM maps tables and relationships into code and helps compose common queries. It does not decide transaction boundaries, prevent N+1 reads, enforce authorization, or guarantee that a generated query is efficient. Entities are persistence types; the rest of the application should receive domain/read-model types shaped for its use case.

## Build it in milestones

### 1. Model three simple entities

Begin with organizations, users, and tickets. Define keys, required/optional fields, relations, and one composite uniqueness rule. Insert and retrieve a ticket through SeaORM before adding labels or comments.

**Checkpoint:** a test loads the ticket and its optional assignee; a ticket from the other organization is not visible through the organization-scoped function.

### 2. Add many-to-many and one-to-many relations

Add labels, the ticket-label join, and comments. Seed a fixture designed to expose duplicate rows and missing relations. Load one detail view and record the number and text of queries.

**Checkpoint:** the view has each label once, ordered comments, and a bounded query count rather than one query per ticket.

### 3. Build the paginated summary

Return a dedicated `TicketSummary`, not entity graphs. Filter by state/assignee/label and sort with a unique tie-breaker. Decide whether aggregation happens in one grouped query, a subquery, or a bounded two-query approach, then inspect the emitted SQL.

**Checkpoint:** equal activity timestamps paginate without duplicate/missing tickets and comment counts remain correct across label joins.

### 4. Add writes and a transaction

Create an organization, initial user, and default label in one transaction. Update tickets with a version predicate; archive them without making old cursors or relations inconsistent.

**Checkpoint:** an injected failure rolls everything back and two stale writers cannot both update the ticket.

### 5. Compare explicit SQL

Implement the summary once with SQLx. Compare call-site clarity, generated SQL, mapping, type checking, pagination, and transaction control. Read Diesel's schema/query-builder overview and describe how its trade-off differs; do not build a third implementation.

**Checkpoint:** both implementations pass the same fixture-driven contract test and your decision record names a concrete situation for each choice.

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

These are graduation tests after the milestone checkpoints pass.

- `T01` — Entity relations load the correct graph for multiple tickets and labels.
- `T02` — The list endpoint executes a bounded number of queries independent of row count.
- `T03` — A failed multi-entity create leaves no partial rows.
- `T04` — Concurrent stale updates cannot overwrite the newest ticket version.
- `T05` — Organization scoping is present in entity and raw-query paths.
- `T06` — Pagination remains stable across joins and equal sort keys.
- `T07` — SeaORM and SQLx implementations produce the same summary values.

## Completion evidence

Submit the entity graph, captured SQL, query-count evidence, and a one-page decision record: “When this codebase should use SeaORM, SQLx, or a database view.” You should finish able to use an ORM without mistaking it for a transaction or authorization model.
