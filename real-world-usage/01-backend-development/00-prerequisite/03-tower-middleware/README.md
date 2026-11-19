# 03 — Tower middleware and traffic control

## Goal

Turn `taskboard-api` into a bounded service under slow clients, traffic bursts, and overloaded dependencies. Learn Tower's `Service`/`Layer` model and prove that middleware order matches policy.

Use the [Tower crate documentation](https://docs.rs/tower/latest/tower/), [`ServiceBuilder`](https://docs.rs/tower/latest/tower/struct.ServiceBuilder.html), [Tower limits](https://docs.rs/tower/latest/tower/limit/), [load shedding](https://docs.rs/tower/latest/tower/load_shed/), [timeouts](https://docs.rs/tower/latest/tower/timeout/), and [`tower-governor`](https://docs.rs/tower_governor/latest/tower_governor/).

## Before you start

Use your completed `taskboard-api` from challenge 02. Its routes and application tests should pass before middleware changes. You should be able to identify a request, response, handler, and shared state. You are not expected to understand Tower's generic types on first reading.

Add one policy at a time. Run the existing tests after each layer; a large middleware stack added in one edit produces type errors and behavior that are hard to localize.

## Mental model

Tower models a component as `request -> future of response`. A `Layer` wraps a `Service`, producing another service. If layers A, B, and C wrap a handler, request observations happen outside-in and response observations happen inside-out. A layer's location therefore controls whether it sees authentication failures, timeouts, and shed requests.

Draw the stack before coding it. Annotate each layer with its rate-limit key, timeout scope, error type, and whether it observes the final status. Keep this drawing current as tests change the order.

## Build it in milestones

### 1. Request identity and spans

Accept a valid, length-bounded request ID or generate one. Put it in request extensions, the root tracing span, the JSON error envelope, and the response header. Use route templates in fields, not raw paths.

**Checkpoint:** success, handler error, and unknown-route responses each return one request ID matching the captured log span.

### 2. Separate public and authenticated traffic

Keep health routes in a small public router. On business routes, replace the temporary user-header extractor with middleware that validates the same learning credential and inserts a typed principal. This is still a fake credential; cryptography belongs to challenge 06.

**Checkpoint:** health works without identity; business routes receive a principal; invalid credentials never enter the handler.

### 3. Bound time and concurrency

Add a deliberately slow test route. Apply a short business timeout, a global in-flight limit, and prompt load shedding. Return stable JSON for timeout and overload errors. Do not add buffering yet.

**Checkpoint:** a burst test shows the cap is never exceeded, excess work is rejected promptly, and health stays responsive.

### 4. Add keyed quotas

Rate-limit unauthenticated attempts by the socket peer address. Rate-limit accepted requests by principal. Only honor forwarded client addresses when the immediate peer is in an explicit trusted-proxy list.

**Checkpoint:** two principals sharing one IP have separate authenticated budgets; a forged forwarding header from an untrusted address changes nothing.

### 5. Prove layer order

Write a table of expected observations, then create focused tests. Temporarily swap tracing/timeout and authentication/quota order and note the changed result. Add your final order to the README with the reason for every adjacent pair.

**Checkpoint:** the tests fail under at least one intentionally wrong order and pass under the documented order.

### 6. Write one small layer

Implement a custom response-header or timing layer rather than another security feature. Its purpose is to make `Layer`, `Service`, readiness, and future wrapping concrete.

**Checkpoint:** unit-test the custom service without starting Axum.

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

These are graduation tests after the milestone checkpoints pass.

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
