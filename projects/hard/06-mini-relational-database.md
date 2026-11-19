# Hard 06 — Mini relational database

## Project challenge

Build an embedded relational database for a deliberately small query language. The challenge is to connect typed schemas, storage pages, indexes, and query execution without hiding inconsistencies.

## Required behavior

- Support creating tables with integer, text, and boolean columns.
- Implement `INSERT`, primary-key lookup, filtered `SELECT`, `UPDATE`, and `DELETE`.
- Validate column names, types, nullability, and primary-key uniqueness.
- Persist table schemas and rows across restarts.
- Use a simple primary-key index rather than scanning for every lookup.
- Recover safely from an incomplete mutation using a journal or copy-on-write strategy.

## Example commands

```text
CREATE TABLE users (id INT PRIMARY KEY, name TEXT, active BOOL);
INSERT INTO users VALUES (1, "Amina", true);
SELECT name FROM users WHERE id = 1;
-> Amina
```

## Architecture expectations

Separate parsing, logical commands, schema validation, execution, row encoding, indexes, and durable storage. State explicit limits instead of imitating all of SQL.

## Rust skills combined

Parsing, type systems, binary encoding, indexes, page or record storage, journaling, recovery, iterators, modules, and property-style tests.

## Done when

Type errors do not mutate storage, restart preserves data and indexes, incomplete writes recover consistently, and query tests cover valid plus invalid syntax.

## System-design foundation

Write schema and transaction guarantees explicitly, inject crashes around journal boundaries, and
measure scans versus indexed lookups as data grows. Later services rely on understanding what their
database does—and does not—make atomic.

### Foundation evidence

- `F01` — Crash at every journal/page publication boundary and compare recovered tables/indexes with a reference model.
- `F02` — Benchmark scan and indexed lookup as rows grow by powers of ten; report p50/p95/p99, file size, and cache state.

## Brown Rust Book prerequisites

Study [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [generic abstractions](https://rust-book.cs.brown.edu/ch10-00-generics.html), [patterns](https://rust-book.cs.brown.edu/ch19-00-patterns.html), and [advanced types](https://rust-book.cs.brown.edu/ch20-03-advanced-types.html).

## Suggested tests

### Unit tests

- `T01` — Parse each supported command and reject invalid syntax.
- `T02` — Schema validation rejects wrong types and duplicate primary keys.
- `T03` — Row encoding round-trips every supported value type.
- `T04` — Primary-key lookup uses the index and returns the correct row.

### Integration tests

- `T05` — The example create, insert, and select flow returns `Amina`.
- `T06` — Restart preserves rows, schemas, and index behavior.
- `T07` — An interrupted mutation recovers to either the old or complete new state.

## Stretch goals

Add secondary indexes, transactions, joins, or a query-plan display.
