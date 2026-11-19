# Messaging and streaming projects

Build durable asynchronous data movement in Rust and make ordering, delivery, replay, lag, and
backpressure observable rather than treating a managed broker as magic.

| Project | Scenario | Core delivery |
| --- | --- | --- |
| [01 — EventLog](01-eventlog/README.md) | Product analytics platform | Partitioned durable log with consumer groups and replay |

Start with [DurableQueue](../02-cloud-infrastructure/05-durablequeue/README.md) if work distribution,
leases, and redelivery are unfamiliar. EventLog differs: multiple consumers retain independent
positions in a durable history.
