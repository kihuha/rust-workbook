# 03 — Cloud infrastructure: Layer 4 load balancer

## What this teaches

A Layer 4 load balancer routes transport connections using IP addresses and ports. It does not need to understand HTTP methods, paths, headers, or response codes.

Build `l4-balancer`, an asynchronous TCP proxy in front of eight dummy HTTP nodes. You will use `curl` to generate recognizable traffic, but the balancer must treat the traffic as an opaque byte stream.

## Topology

```text
curl http://127.0.0.1:8080/demo
                |
                v
       l4-balancer :8080
          TCP connection
                |
       +--------+--------+
       |        |        |
    node-1   node-2   ... node-8
     :9101    :9102       :9108
       |        |            |
 node-1.log node-2.log ... node-8.log
```

Each dummy node is a tiny HTTP server that returns its node ID and appends one line per request to its own file. Keep HTTP parsing inside the dummy nodes. The Layer 4 balancer only accepts a TCP connection, selects a healthy backend, connects to it, and copies bytes in both directions.

## Build

Use a TOML or YAML file to configure the listen address, backend socket addresses, health-check interval, connection timeout, and balancing algorithm.

Support these algorithms behind a shared Rust trait:

1. Round robin.
2. Random selection.
3. Least active connections.
4. Source-IP hash for simple connection affinity.
5. Weighted round robin as an extension.

A configuration value such as `algorithm = "least-connections"` selects the implementation at startup. Do not create a separate application for every algorithm; keeping the interface stable makes their behavior easier to compare.

## Suggested milestones

1. Build one dummy node that identifies itself and appends to its log.
2. Run eight nodes on ports `9101` through `9108`.
3. Proxy one TCP connection to one fixed node with bidirectional copying.
4. Add round-robin backend selection.
5. Extract a `LoadBalancingStrategy` trait and add the other algorithms.
6. Add TCP health checks, connect timeouts, connection counters, and graceful drain.

## Rules that preserve the Layer 4 lesson

- Do not inspect the HTTP path, method, headers, or status in the balancer.
- Make one backend decision per accepted client connection.
- Do not retry a connection after bytes may have reached a backend; doing so can duplicate application work.
- Bound connection count and all internal queues.
- Remove unhealthy nodes from new selection while allowing existing connections to finish.
- On shutdown, stop accepting new connections and give active connections a deadline to drain.

## Observe and compare

Run repeated requests and inspect the node log files:

```bash
for i in {1..24}; do curl -s http://127.0.0.1:8080/demo; done
```

Then keep a connection open, stop a node, skew node weights, and make requests from different source addresses where possible. Explain why request counts may not be perfectly even when balancing connections rather than HTTP requests.

## Required tests

- Round robin visits all healthy nodes in order.
- Least-connections chooses a node with the smallest active count.
- A node marked unhealthy receives no new connections.
- Bidirectional copying preserves arbitrary bytes and half-close behavior.
- Connection counters are released on success, error, timeout, and cancellation.
- A graceful drain stops new accepts and bounds shutdown time.

## Done when

All eight nodes can run locally, `curl` traffic is visibly distributed among their log files, algorithms can be changed through configuration, and you can explain what the balancer knows—and cannot know—at Layer 4.

## System-design practice

Define connection rate, concurrent connections, byte distribution, keep-alive duration, backend
latency, slow clients, and host/network limits. Measure accept/connect p95/p99, goodput, errors,
active connections, memory, and rejection past saturation. Compare balancing algorithms and active
versus passive health without adding HTTP policy. Senior proves bounded TCP forwarding; Staff adds
replicas, connection draining, hot-backend protection, compatible reload, and failure budgets;
Principal adds regional placement, capacity reserve, cost/GiB, ownership, migration, and rollback.
Review [Google SRE cascading failures](https://sre.google/sre-book/addressing-cascading-failures/).

## Extensions

Add consistent hashing, PROXY protocol, TLS passthrough, per-backend connection limits, Docker Compose, latency metrics, or a small traffic generator.
