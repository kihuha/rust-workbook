# 04 — Cloud infrastructure: Layer 7 load balancer

## What this teaches

A Layer 7 load balancer understands an application protocol. For HTTP, it can make a decision for each request using the host, path, method, or headers, even when several requests reuse one client connection.

Build `l7-balancer`, an HTTP reverse proxy in front of the same eight dummy nodes used by the Layer 4 project. Reusing the harness makes the extra information and policy at Layer 7 easy to see.

## Topology

```text
curl -H 'X-Tenant: blue' http://127.0.0.1:8080/images/1
                              |
                              v
                    l7-balancer :8080
                    parse HTTP request
                    choose route + node
                              |
             +----------------+----------------+
             |                                 |
       /api -> node pool A              /images -> node pool B
             |                                 |
       node-1.log ... node-4.log        node-5.log ... node-8.log
```

Use an established HTTP library such as Hyper; the learning goal is proxy and routing behavior, not writing an HTTP parser.

## Build

Load routes, backend pools, health settings, and a default balancing algorithm from TOML or YAML. Support:

- host-based and path-prefix routing;
- round robin, random, least active requests, weighted round robin, and header-hash selection behind a shared trait;
- per-request backend selection;
- streaming request and response bodies rather than buffering them without a limit;
- forwarding-header sanitization and a generated request ID;
- connect, header, and overall request timeouts;
- active health checks and passive failure tracking; and
- graceful drain and valid configuration reload.

The dummy nodes append node ID, request ID, method, path, and timestamp to `node-1.log` through `node-8.log`. This information is written by the application-level dummy nodes, not by the Layer 4 balancer.

## Example configuration

```toml
listen = "127.0.0.1:8080"
algorithm = "least-requests"

[[routes]]
path_prefix = "/api"
pool = "api"

[[routes]]
path_prefix = "/images"
pool = "images"

[pools]
api = ["127.0.0.1:9101", "127.0.0.1:9102", "127.0.0.1:9103", "127.0.0.1:9104"]
images = ["127.0.0.1:9105", "127.0.0.1:9106", "127.0.0.1:9107", "127.0.0.1:9108"]
```

## Suggested milestones

1. Reuse the eight-node harness and proxy one HTTP request to one node.
2. Select a backend independently for every request.
3. Add round robin through a `LoadBalancingStrategy` trait.
4. Add the remaining algorithms and switch them through configuration.
5. Add host/path routes and multiple backend pools.
6. Add streaming, health tracking, timeouts, reload, and graceful drain.

## Policy and safety rules

- Retry only requests that are safe and replayable, and only before an ambiguous outcome.
- Strip or replace untrusted forwarding and hop-by-hop headers.
- Bound request bodies, headers, concurrency, queues, and timeout durations.
- Do not send new requests to unhealthy nodes; use recovery thresholds to avoid flapping.
- Reject an invalid reload and keep the last valid configuration active.
- Propagate client cancellation so backend work and permits are released.

## Compare with Layer 4

Run the same request sequence through both projects and record:

- whether balancing happens per connection or per HTTP request;
- which request details are visible to the balancer;
- whether `/api` and `/images` can use different pools;
- which retry decisions are possible;
- how keep-alive changes the observed distribution; and
- the complexity and overhead added by parsing the application protocol.

Do not conclude that Layer 7 is simply better. The goal is to understand why Layer 4 is simpler and protocol-agnostic, while Layer 7 enables richer policy.

## Required tests

- The most specific host/path route selects the correct pool.
- Every strategy selects only healthy nodes.
- Least-requests counters are released on every completion path.
- Multiple requests on one keep-alive connection may reach different nodes.
- Streaming a large body stays within the configured memory bound.
- Unsafe or non-replayable requests are not retried.
- A bad configuration reload leaves current routing unchanged.

## Done when

Traffic is visibly routed among the eight node log files, routes and algorithms are configurable, and a short comparison explains the behavioral boundary between the Layer 4 and Layer 7 implementations.

## System-design practice

Declare route mix, request/response sizes, keep-alive reuse, upstream latency, concurrency, and
resource limits. Report added p50/p95/p99, goodput, memory, retry rate, and overload rejection.
Senior implements bounded HTTP routing; Staff adds per-tenant quotas, adaptive concurrency, retry
budgets, circuit breaking, canary reload, and old/new configuration compatibility; Principal adds
regional routing, residency, capacity/cost strategy, policy ownership, and staged migration. Keep
HTTP-library choice fixed for the lesson while treating admission, retry, health, and configuration
semantics as open decisions. Use [Tower](https://docs.rs/tower/latest/tower/) as research context.

## Extensions

Add cookie affinity, canary weights, circuit breaking, WebSockets, TLS termination, Prometheus metrics, Docker Compose, or benchmark both balancers under the same workload.
