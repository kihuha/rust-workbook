# 00 — Learn Rust backend development

This is a guided bridge between knowing the Rust language and building the larger backend projects in this workbook. It is not a list of technologies to read about. You will build several focused exercises, grow a small task service through the core stack, and finish by applying the same ideas to a new service.

Do not begin with PayForge or BankBridge and look up every unfamiliar concept at once. Complete the **foundation** challenges first, then add the **production** concerns. Each brief has small checkpoints; stop at a checkpoint, run its tests, and commit it before moving on.

SwiftCart is a separate, protocol-first route. It uses standard-library TCP and manual HTTP parsing to show what a web framework automates. You do not need SwiftCart before this track.

## What you should know first

This track teaches backend engineering libraries, not the Rust language from zero. Before challenge 01, you should be able to:

- create a Cargo package, add a dependency, run a binary, and run tests;
- use structs, enums, pattern matching, modules, traits, generics, and iterators;
- explain ownership and borrowing, use `String`, `&str`, `Option`, and `Result`, and follow a compiler lifetime error;
- define a custom error enum and use `?` without relying on `unwrap()` in normal failure paths;
- write unit and integration tests; and
- use Git well enough to make a commit at each checkpoint.

The matching workbook chapters are [packages and modules](../../../chapters/07-packages-crates-and-modules.md), [error handling](../../../chapters/09-error-handling.md), [traits and lifetimes](../../../chapters/10-generics-traits-and-lifetimes.md), [tests](../../../chapters/11-writing-automated-tests.md), and [async Rust](../../../chapters/17-async-await-futures-and-streams.md). Async Rust may still feel new; challenge 01 teaches its backend-specific use.

You also need basic, language-independent familiarity with HTTP methods/status codes, JSON, relational tables/keys, and a terminal. You do **not** need prior experience with Axum, Tokio, Tower, SQLx, an ORM, JWTs, OpenTelemetry, Kubernetes, or distributed systems.

### Readiness exercise

Before starting, make a tiny CLI that parses two arguments, constructs a value, serializes it to JSON, returns a useful error for invalid input, and has two tests. If that takes more than a study session, revisit the linked chapters first. This is a diagnostic, not another portfolio project.

## Local tools

Install a current stable Rust toolchain, Git, `curl`, and Docker with Compose (or a compatible container engine). PostgreSQL is only required from challenge 04 onward. Verify the baseline with:

```console
rustc --version
cargo --version
git --version
curl --version
docker compose version
```

Use a dedicated repository or a `backend-learning` directory for your solutions. Keep every challenge as a separate Cargo package until challenge 09; this makes the lesson boundaries visible. Pin the generated `Cargo.lock`, record your `rustc --version`, and use crate versions that are mutually compatible rather than copying an old lockfile from the workbook.

## How to use a brief

For each challenge:

1. Read only its **Before you start**, **Mental model**, and first milestone.
2. Build the milestone and run its checkpoint before reading ahead.
3. Keep domain logic separate from framework code when the brief asks for that boundary.
4. Add the named graduation tests after the small checkpoints work.
5. Write the completion evidence in the project README in your own words.

The briefs deliberately specify behavior without giving a complete solution. “Suggested shape” is a starting structure, not an architecture rule. When a production topic would distract from the current lesson, the brief provides an in-memory fake or postpones it.

## Learning path

Times are rough study estimates, not deadlines. One “session” means roughly 60–90 focused minutes.

| Phase | Challenge | What changes | Typical effort |
| --- | --- | --- | --- |
| Foundation | [01 — Tokio runtime](01-tokio-runtime/README.md) | A synchronous job runner becomes bounded async work | 3–5 sessions |
| Foundation | [02 — Axum API](02-axum-api/README.md) | The in-memory application gains an HTTP boundary | 4–6 sessions |
| Foundation | [03 — Tower middleware](03-tower-middleware/README.md) | The API becomes bounded under bad or excessive traffic | 3–5 sessions |
| Foundation | [04 — SQLx + PostgreSQL](04-sqlx-postgres/README.md) | Memory is replaced with durable transactional storage | 5–8 sessions |
| Production | [05 — SeaORM](05-seaorm-data-modeling/README.md) | The same persistence problems are viewed through an ORM | 3–5 sessions |
| Production | [06 — Auth](06-authentication-authorization/README.md) | Requests receive identities and tenant-safe permissions | 5–8 sessions |
| Production | [07 — Outbound reliability](07-outbound-reliability/README.md) | The service calls a dependency that can fail ambiguously | 4–7 sessions |
| Production | [08 — Operations](08-observability-operations/README.md) | Failures become diagnosable and shutdown becomes deliberate | 4–6 sessions |
| Readiness gate | [09 — Dispatch Hub](09-integration-capstone/README.md) | The concepts are combined in a fresh, bounded service | 10–16 sessions |

Challenges 01–04 are the minimum foundation for an ordinary Rust CRUD API. Challenges 05–08 teach concerns required by the production-style projects in this workbook. SeaORM is a comparison lesson: complete it, but do not assume every later service must use an ORM.

## The service you grow in challenges 01–04

The first challenge teaches job execution in isolation; challenges 02–04 grow one task-management service so each lesson introduces one major variable:

```text
HTTP client -> Axum routes -> application service -> repository
                    |
              Tower middleware

01: job execution only
02: in-memory repository + HTTP
03: same API + traffic policy
04: PostgreSQL repository
```

Start each challenge from the previous result when the brief says to. Keep a Git tag or branch at each boundary so you can compare the uninstrumented and instrumented versions.

## Shared working standard

Every finished challenge should have:

- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` passing;
- no `unwrap()` or `expect()` on input, network, database, configuration, or shutdown paths;
- explicit limits for queues, bodies, connections, retries, and timeouts introduced in that lesson;
- typed errors at application boundaries and stable errors at the HTTP boundary;
- tests at the cheapest useful layer, with real disposable PostgreSQL or network servers where their behavior is the subject; and
- a short README containing run commands, design choices, known limitations, and checkpoint evidence.

The advanced “required tests” are graduation tests. They are not where you start. The milestone checkpoints tell you the smaller feedback loop that gets you there.

## Getting help without skipping the lesson

When stuck, reduce the problem to the current boundary: test the domain without HTTP, the router without a socket, SQL against a disposable database, or a connector against Wiremock. Read the compiler error from the first relevant line and make one change. Ask for an explanation of the error or a smaller example before asking for a finished project.

The primary-source map is [Rust backend framework documentation](../../../research/rust-backend-frameworks.md). Each brief links only the sections relevant to its lesson.

## Exit criteria

You are ready for PayForge or BankBridge when you can follow an authenticated request through middleware, an application service, a transaction, and an outbound call; prevent cross-tenant access in the query layer; explain your overload limits; diagnose a forced failure from telemetry; and shut down without losing acknowledged work. Challenge 09 makes you demonstrate those skills rather than merely claim them.
