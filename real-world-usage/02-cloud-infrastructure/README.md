# Cloud infrastructure projects

Learn where Rust fits in cloud and infrastructure work by building long-running agents, network data planes, control loops, and operational tooling. These projects are not all web APIs: several are deliberately small infrastructure daemons whose behavior is explored through configuration, logs, processes, sockets, and failure injection.

## Projects

The smaller projects keep the surrounding product simple so the infrastructure concept remains visible. NimbusFleet then combines several cloud-infrastructure concerns into a larger production-style system.

| Project | Concept taught | What you run |
| --- | --- | --- |
| [01 — NimbusFleet](01-nimbusfleet/README.md) | Reconciliation, desired state, identity, and recovery | A regional container deployment controller |
| [02 — Service health and endpoint monitor](02-service-health-monitor/README.md) | Scheduling, health state, timeouts, telemetry, and alerting | One config-driven monitoring daemon |
| [03 — Layer 4 load balancer](03-layer-4-load-balancer/README.md) | TCP forwarding, connection balancing, health, and graceful shutdown | A TCP proxy and eight dummy nodes |
| [04 — Layer 7 load balancer](04-layer-7-load-balancer/README.md) | HTTP-aware routing, request balancing, and policy | An HTTP proxy and eight dummy nodes |

The two load-balancer projects intentionally use the same dummy-node harness. A `curl` request passes through the balancer, and the selected node appends a record to `node-1.log` through `node-8.log`. Comparing the implementations shows what information and policy become available when a balancer moves from Layer 4 to Layer 7.

Add future cloud-infrastructure briefs as sibling project directories with their own `README.md`.
