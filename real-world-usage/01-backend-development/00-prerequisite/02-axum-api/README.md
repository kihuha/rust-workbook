# 02 — Axum API fundamentals

## Goal

Build `taskboard-api`, an in-memory multi-user task service using Tokio, Axum, and Serde. Concentrate on the HTTP boundary: routes, typed extraction, shared state, validation, errors, and tests.

Use the [Axum crate documentation](https://docs.rs/axum/latest/axum/), [Axum error-handling documentation](https://docs.rs/axum/latest/axum/error_handling/), [Serde](https://serde.rs/), [`validator`](https://docs.rs/validator/latest/validator/), and the official [Axum testing example](https://github.com/tokio-rs/axum/tree/main/examples/testing).

## Before you start

Finish challenge 01 through bounded admission and shutdown. Be comfortable with `async fn`, `Arc`, `Result`, Serde derives, and Tokio tests. You need basic HTTP vocabulary—method, path, header, body, and status—but not prior web-framework experience.

Create a new `taskboard-api` package. Reuse `jobdeck`'s shutdown idea only; task creation is ordinary request work, not a background job. Add Axum, Serde/`serde_json`, Tokio, Tower's utility feature for tests, a UUID/ULID crate, and `validator`. Keep storage in memory so this lesson remains about HTTP.

## What you are building

A user creates and manages simple tasks:

```json
{
  "title": "Read the Axum extractor docs",
  "description": "Stop after the JSON example",
  "status": "todo"
}
```

A stored task adds `id`, `owner_id`, `version`, `created_at`, and `updated_at`. For now, a bounded `x-user-id` header supplies the owner. Real authentication arrives in challenge 06. Never accept `owner_id`, IDs, versions, or timestamps from a create body.

## Mental model

Axum routes a request to a handler. Extractors turn transport data into typed inputs before the handler runs; `IntoResponse` turns success or failure into HTTP. The handler should translate between HTTP and an application service, not contain the business rules. An in-process router test is still an HTTP-boundary test even though it does not bind a socket.

## Suggested shape

```text
src/
  main.rs          # listener and shutdown only
  lib.rs           # app(state) -> Router
  http/{routes,error,dto}.rs
  application.rs   # task use cases; no Axum types
  domain.rs
  repository.rs    # in-memory implementation
tests/api.rs
```

If defining an async repository trait becomes the main difficulty, use concrete `MemoryTaskRepository` methods in this challenge and extract the trait in challenge 04.

## Build it in milestones

### 1. Serve one typed route

Build `GET /health/live`, bind `127.0.0.1:3000`, and call it with `curl`. Move router construction into `lib.rs` immediately so tests can construct the same app.

**Checkpoint:** a `ServiceExt::oneshot` test receives `200`; a manual `curl` receives the same small JSON document.

### 2. Create and retrieve tasks

Define separate `CreateTaskRequest`, domain `Task`, and `TaskResponse` types. Validate title length and reject unknown JSON fields if that is your declared contract. Implement `POST` and `GET` using an application service and in-memory repository.

**Checkpoint:** create returns `201`, a `Location` header, and version 1; following that location returns the same task. Missing IDs return your JSON error envelope.

### 3. Centralize errors and rejection behavior

Create `AppError` with explicit variants and one `IntoResponse` implementation. Include a request ID, but use a temporary generated value until challenge 03 owns request IDs. Add body and content-type limits. Verify framework rejections use the same bounded public shape or document the intentional exception.

**Checkpoint:** malformed JSON, invalid fields, oversized JSON, missing user header, and unknown task IDs have different stable `code` values.

### 4. Add list, patch, and archive

Sort by `(created_at, id)` and make the opaque cursor encode both values. Require the expected version on update; increment it only on success. Archive rather than physically delete, and define whether repeating archive returns `204` or the archived representation.

**Checkpoint:** two equal-time records paginate without repeat/skip; a stale patch returns `409` and changes nothing.

### 5. Exercise a real socket

Bind an ephemeral listener in one smoke test, serve the same router, call it with Reqwest, then cancel and await the server. Keep most tests in process because they are faster and produce narrower failures.

**Checkpoint:** the socket test creates and reads one task and leaves no detached server task.

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

These are graduation tests after the milestone checkpoints pass.

- `T01` — Create and retrieve a valid task using `ServiceExt::oneshot` without binding a socket.
- `T02` — Invalid JSON, invalid fields, oversized bodies, and wrong content types have distinct stable errors.
- `T03` — Unknown routes and rejected methods return bounded JSON errors.
- `T04` — Cursor pagination is stable when multiple records share a timestamp.
- `T05` — A stale update version returns `409` and changes nothing.
- `T06` — Ten concurrent updates preserve the documented invariant.
- `T07` — A Reqwest smoke test exercises the same router over a real ephemeral listener.

## Completion evidence

Provide the route tree, error catalogue, OpenAPI sketch, and test matrix. Explain why an Axum handler error must still become a valid response at the serving boundary and where framework types stop in your architecture.
