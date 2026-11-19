# 02 — Axum API fundamentals

## Goal

Build `taskboard-api`, an in-memory multi-user task service using Tokio, Axum, and Serde. Concentrate on the HTTP boundary: routes, typed extraction, shared state, validation, errors, and tests.

Use the [Axum crate documentation](https://docs.rs/axum/latest/axum/), [Axum error-handling documentation](https://docs.rs/axum/latest/axum/error_handling/), [Serde](https://serde.rs/), [`validator`](https://docs.rs/validator/latest/validator/), and the official [Axum testing example](https://github.com/tokio-rs/axum/tree/main/examples/testing).

## Required API

| Method | Endpoint | Behavior |
| --- | --- | --- |
| `POST` | `/v1/tasks` | Validate and create a task |
| `GET` | `/v1/tasks/{id}` | Return one task or a stable not-found error |
| `GET` | `/v1/tasks` | Cursor-paginate and filter by status |
| `PATCH` | `/v1/tasks/{id}` | Update allowed fields with optimistic version checking |
| `DELETE` | `/v1/tasks/{id}` | Idempotently archive a task |
| `GET` | `/health/live` | Return process liveness |

## Framework requirements

- Compose nested `Router`s and use `State` for an application service, not for scattered globals.
- Create typed path, query, header, and JSON extractors.
- Separate deserialization, syntactic validation, authorization placeholders, and domain validation.
- Implement one application error type that converts through `IntoResponse` to `{code,message,request_id,details}`.
- Configure a small JSON body limit and reject unsupported content types.
- Return explicit status codes and headers, including `Location` on create and `ETag` or version metadata on update.
- Keep handlers thin; domain operations must be callable without Axum types.

## Required tests

- `T01` — Create and retrieve a valid task using `ServiceExt::oneshot` without binding a socket.
- `T02` — Invalid JSON, invalid fields, oversized bodies, and wrong content types have distinct stable errors.
- `T03` — Unknown routes and rejected methods return bounded JSON errors.
- `T04` — Cursor pagination is stable when multiple records share a timestamp.
- `T05` — A stale update version returns `409` and changes nothing.
- `T06` — Ten concurrent updates preserve the documented invariant.
- `T07` — A Reqwest smoke test exercises the same router over a real ephemeral listener.

## Completion evidence

Provide the route tree, error catalogue, OpenAPI sketch, and test matrix. Explain why an Axum handler error must still become a valid response at the serving boundary and where framework types stop in your architecture.
