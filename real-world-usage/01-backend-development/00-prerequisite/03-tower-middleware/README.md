# 03 — Tower middleware and traffic control

## Goal

Turn `taskboard-api` into a bounded service under slow clients, traffic bursts, and overloaded dependencies. Learn Tower's `Service`/`Layer` model and prove that middleware order matches policy.

Use the [Tower crate documentation](https://docs.rs/tower/latest/tower/), [`ServiceBuilder`](https://docs.rs/tower/latest/tower/struct.ServiceBuilder.html), [Tower limits](https://docs.rs/tower/latest/tower/limit/), [load shedding](https://docs.rs/tower/latest/tower/load_shed/), [timeouts](https://docs.rs/tower/latest/tower/timeout/), and [`tower-governor`](https://docs.rs/tower_governor/latest/tower_governor/).

## Required middleware

- Generate or validate a bounded request ID and return it in every response.
- Create one root tracing span without recording authorization or request bodies.
- Limit request-body size before parsing.
- Authenticate a temporary test principal and attach it through request extensions.
- Apply separate per-IP unauthenticated and per-principal authenticated quotas.
- Enforce a global in-flight concurrency limit and shed excess load with a stable `503`.
- Apply route-specific timeouts; health endpoints must not inherit slow business-route timeouts.
- Catch middleware errors and convert them to the common JSON error envelope.

## Design constraints

Write down the layer order from outside to inside and what each layer observes. Define trusted-proxy behavior before reading forwarded client IP headers. Do not retry incoming requests inside generic middleware. Keep queues bounded; a buffer may move backpressure but must not hide overload.

## Required experiments and tests

- `T01` — Authentication failures consume the intended IP quota but never a principal quota.
- `T02` — Two principals behind one trusted proxy receive independent authenticated quotas.
- `T03` — Forged forwarding headers from an untrusted peer do not change the rate-limit key.
- `T04` — Timeout responses retain request IDs and trace context.
- `T05` — The concurrency cap holds during a burst of slow requests.
- `T06` — Load shedding rejects promptly instead of growing an unbounded queue.
- `T07` — Swapping timeout and trace layers changes a test-observable result; document which order is correct.
- `T08` — Metrics and logs contain route templates, never raw object IDs or bearer tokens.

## Completion evidence

Submit a middleware-order diagram, quota policy, overload test results, and one custom `Layer` or `Service` implementation. Explain readiness/backpressure, timeout scope, and the difference between rate, concurrency, buffering, and load shedding.
