# 01 — WebAssembly: LedgerLens browser analytics

## Company and application

LedgerLens is a fictional accounting-software company. Customers need to explore large CSV exports in the browser without uploading sensitive financial data to a server.

Build a Rust analytics engine compiled to WebAssembly and a small TypeScript interface running it inside a Web Worker. The application imports, validates, filters, groups, and exports data locally.

## Brown Rust Book prerequisites

Study [ownership and slices](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html), and [advanced types](https://rust-book.cs.brown.edu/ch20-03-advanced-types.html).

## User workflow

1. Select a local CSV file.
2. Preview detected columns and inferred types.
3. Confirm or correct the schema.
4. Apply filters and group-by aggregations.
5. View a paged result and summary chart.
6. Export results as CSV without sending source data over the network.

## Rust/Wasm API

Expose a small stable interface rather than one function per UI control.

```text
create_session(options) -> session_id
push_chunk(session_id, bytes, is_final) -> progress
schema(session_id) -> column definitions and parse warnings
execute(session_id, query) -> result_id and summary
page(result_id, offset, limit) -> rows
export_chunk(result_id, cursor, max_bytes) -> bytes and next cursor
cancel(operation_id)
drop_session(session_id)
```

Use serializable versioned request and response structures. Avoid repeatedly copying entire datasets across the JavaScript/Wasm boundary.

## Supported query scope

- Typed columns: string, signed integer, decimal minor units, boolean, and date.
- Filters: equality, numeric/date range, contains, and missing value.
- Grouping by up to three columns.
- Aggregations: count, sum, minimum, maximum, and mean.
- Stable sorting and paged result access.
- Explicit null and malformed-value policy.

Do not implement arbitrary JavaScript expressions or dynamic code execution.

## Example input

```csv
date,category,amount_minor,currency
2026-11-01,food,800,KES
2026-11-01,transport,150,KES
2026-11-02,food,4500,KES
```

## Example query and result

```json
{"filter":{"currency":"KES"},"group_by":["category"],"aggregates":[{"sum":"amount_minor"}],"sort":[{"field":"sum_amount_minor","direction":"desc"}]}
```

```text
food: 5300
transport: 150
rows processed: 3
parse errors: 0
```

## Browser requirements

- Parse in bounded chunks inside a Web Worker so the UI remains responsive.
- Report progress and allow cancellation.
- Enforce configurable file, row, column, and output limits.
- Release Wasm memory when sessions and results are dropped.
- Support current evergreen desktop browsers with a documented compatibility matrix.
- Meet accessibility requirements for import errors, tables, controls, and charts.

## Privacy and security

- The production application must work without uploading imported content.
- Use a strict Content Security Policy and no dynamic code evaluation.
- Escape all displayed and exported values appropriately.
- Avoid analytics that capture file names, column values, or query contents.
- Pin dependencies and publish a software bill of materials.
- Treat malformed CSV and oversized values as untrusted input.

## Required tests

### Rust unit and property tests

- `T01` — Chunked parsing equals parsing the same complete input.
- `T02` — Schema inference handles empty, missing, and conflicting values.
- `T03` — The example groups to food `5300` and transport `150`.
- `T04` — Stable sorting preserves source order for equal keys.
- `T05` — Generated filters and aggregations match a simple reference implementation.

### Browser integration tests

- `T06` — Worker messages complete the import, query, page, and export lifecycle.
- `T07` — Cancellation stops a large operation and the UI remains usable.
- `T08` — Malformed UTF-8 and CSV return bounded diagnostics without a crash.
- `T09` — Session disposal releases retained result handles and memory.
- `T10` — Exported CSV round-trips quoted commas, newlines, and quotes.

### Performance, privacy, and accessibility tests

- `T11` — Process the target file size within the memory and time budgets.
- `T12` — Verify no imported content appears in network requests or telemetry.
- `T13` — Run automated accessibility checks and keyboard-only workflows.
- `T14` — Test supported browsers in CI and record bundle-size budgets.
- `T15` — Fuzz the Rust parser and query decoder natively before Wasm packaging.

## Build and deployment

Produce reproducible optimized Wasm and JavaScript packages with source maps handled according to policy. Enforce bundle-size budgets and cache immutable hashed assets.

Deploy as a static site behind TLS and a strict CSP. Provide version compatibility between the UI and Wasm module, rollback to the previous asset set, and browser-based error reporting without sensitive data.

## Service objectives

- The UI remains interactive during import and query work.
- The target dataset stays within the documented peak-memory budget.
- No source row leaves the browser in normal operation.
- An incompatible cached UI/Wasm pair self-recovers through version detection.

## Acceptance criteria

A user can import the example and target-size datasets, correct schema, query, page, cancel, and export entirely locally in each supported browser with documented performance and accessibility results.

## Optional extensions

Add Parquet import, IndexedDB session persistence, SIMD acceleration, streaming charts, or an embeddable npm package.
