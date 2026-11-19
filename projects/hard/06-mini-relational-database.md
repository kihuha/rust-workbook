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

## Stretch goals

Add secondary indexes, transactions, joins, or a query-plan display.
