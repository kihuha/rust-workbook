# Hard projects

These projects require deliberate architecture, failure recovery, concurrency or protocol design,
and broader automated testing. Build them in milestones. They are deliberately smaller than the
real-world systems so the learner can implement the difficult primitive before organizational,
multi-tenant, multi-region, and operational concerns are added.

## Projects and Brown Rust Book prerequisites

| Project | Study these book chapters or sections first | What the project proves |
| --- | --- | --- |
| [01. Async chat server](01-async-chat-server.md) | [Ch. 15: Shared ownership](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html), [Ch. 16: Channels and shared state](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [Ch. 17: Async concurrency](https://rust-book.cs.brown.edu/ch17-00-async-await.html), [Ch. 18: Trait objects](https://rust-book.cs.brown.edu/ch18-02-trait-objects.html) | You can design a bounded, concurrent protocol service with explicit lifecycle rules. |
| [02. Persistent key-value store](02-persistent-key-value-store.md) | [Ch. 8: Maps](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html), [Ch. 9: Errors](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [Ch. 16: Shared state](https://rust-book.cs.brown.edu/ch16-03-shared-state.html), [Ch. 20: Unsafe contracts](https://rust-book.cs.brown.edu/ch20-01-unsafe-rust.html) if used | You can make acknowledged writes recoverable and compact storage without corrupting state. |
| [03. Durable job scheduler](03-durable-job-scheduler.md) | [Ch. 6: State enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [Ch. 10: Trait boundaries](https://rust-book.cs.brown.edu/ch10-02-traits.html), [Ch. 16: Threads](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [Ch. 17: Tasks and futures](https://rust-book.cs.brown.edu/ch17-06-futures-tasks-threads.html) | You can coordinate time, retries, persistence, cancellation, and crash recovery. |
| [04. Incremental backup tool](04-incremental-backup-tool.md) | [Ch. 4: Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [Ch. 9: Error propagation](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), [Ch. 12: File I/O](https://rust-book.cs.brown.edu/ch12-02-reading-a-file.html), [Ch. 13: Iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html) | You can create atomic, content-addressed snapshots and restore them safely. |
| [05. Expression language interpreter](05-expression-language-interpreter.md) | [Ch. 6: Enums and matching](https://rust-book.cs.brown.edu/ch06-00-enums.html), [Ch. 10: Lifetimes and traits](https://rust-book.cs.brown.edu/ch10-00-generics.html), [Ch. 15: Boxed recursive types](https://rust-book.cs.brown.edu/ch15-01-box.html), [Ch. 19: Patterns](https://rust-book.cs.brown.edu/ch19-00-patterns.html) | You can transform source text into recursive syntax and evaluated values with useful errors. |
| [06. Mini relational database](06-mini-relational-database.md) | [Ch. 8: Collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [Ch. 9: Errors](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [Ch. 10: Generic abstractions](https://rust-book.cs.brown.edu/ch10-00-generics.html), [Ch. 19: Patterns](https://rust-book.cs.brown.edu/ch19-00-patterns.html), [Ch. 20: Advanced types](https://rust-book.cs.brown.edu/ch20-03-advanced-types.html) | You can connect parsing, schemas, indexes, durable mutation, and recovery. |
| [07. Rate-limited HTTP service](07-rate-limited-http-service.md) | [Ch. 7: Public APIs](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [Ch. 10: Traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [Ch. 17: Async](https://rust-book.cs.brown.edu/ch17-00-async-await.html), [Ch. 18: Dynamic abstraction](https://rust-book.cs.brown.edu/ch18-02-trait-objects.html) | You can build thin handlers around tested domain, storage, rate-limit, and shutdown components. |
| [08. Parallel build graph runner](08-parallel-build-graph-runner.md) | [Ch. 8: Collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [Ch. 13: Closures and iterators](https://rust-book.cs.brown.edu/ch13-00-functional-features.html), [Ch. 16: Worker coordination](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [Ch. 19: Destructuring](https://rust-book.cs.brown.edu/ch19-03-pattern-syntax.html) | You can validate a DAG, schedule ready tasks, propagate failure, and cache safely. |
| [09. Resumable file transfer](09-resumable-file-transfer.md) | [Ch. 9: Error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [Ch. 12: File I/O](https://rust-book.cs.brown.edu/ch12-02-reading-a-file.html), [Ch. 16: Concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), [Ch. 17: Async tasks](https://rust-book.cs.brown.edu/ch17-00-async-await.html) | You can resume verified chunks and publish only complete, path-safe files. |
| [10. Event-sourced order system](10-event-sourced-order-system.md) | [Ch. 5: Domain structs](https://rust-book.cs.brown.edu/ch05-00-structs.html), [Ch. 6: Event enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [Ch. 10: Trait boundaries](https://rust-book.cs.brown.edu/ch10-02-traits.html), [Ch. 18: Design tradeoffs](https://rust-book.cs.brown.edu/ch18-05-design-challenge.html), [Ch. 19: Patterns](https://rust-book.cs.brown.edu/ch19-00-patterns.html) | You can enforce commands, replay immutable facts, reject stale writes, and rebuild projections. |

## Readiness check

Start this level after completing two medium projects and the linked later chapters. You should already be able to test failure recovery and reason about concurrent ownership.

## Real-world continuations

| Hard project | Primitive learned | Continue with |
| --- | --- | --- |
| Async chat server | Long-lived sessions, fan-out, slow consumers | [CollabPad](../../real-world-usage/12-real-time-collaboration/01-collabpad/README.md) |
| Persistent KV store | WAL, recovery, compaction | [LogStore](../../real-world-usage/08-databases/03-logstore/README.md), then [ShardStore](../../real-world-usage/08-databases/04-shardstore/README.md) |
| Durable scheduler | State, leases, retries, recovery | [DurableQueue](../../real-world-usage/02-cloud-infrastructure/05-durablequeue/README.md) |
| Incremental backup | Content addressing and atomic publication | Storage snapshots, replication, and restore exercises |
| Mini relational database | Schema, indexes, transactions | PayForge, BankBridge, and distributed storage |
| Rate-limited HTTP service | Admission, quotas, API boundaries | [StreamGate](../../real-world-usage/05-networking/01-streamgate/README.md) and [TenantPlane](../../real-world-usage/02-cloud-infrastructure/07-tenantplane/README.md) |
| Parallel build graph | Dependency scheduling and caching | NimbusFleet reconciliation and workflow orchestration |
| Resumable transfer | Framing, integrity, retry, publication | ShardStore repair and regional migration |
| Event-sourced orders | Immutable history and optimistic concurrency | SwiftCart, PayForge, and EventLog |
