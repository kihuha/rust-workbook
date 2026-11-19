# Backend development projects

Production-style Rust services involving HTTP APIs, domain rules, persistence, authentication, asynchronous work, and operations. If you are learning backend development rather than assessing an existing skill set, begin with the [guided prerequisite track](00-prerequisite/README.md); the larger briefs assume those skills and are intentionally less tutorial-like.

## Learning routes

- **Protocol-first:** build SwiftCart with the standard library, complete the framework prerequisites, then attempt PayForge or BankBridge.
- **Application-first (recommended for backend beginners):** complete the guided framework prerequisites, then attempt PayForge or BankBridge; return to SwiftCart when you want to study what the frameworks automate.

| Project | Company scenario | Core delivery |
| --- | --- | --- |
| [00 — Guided prerequisites](00-prerequisite/README.md) | Focused production-backend exercises | Tokio, Axum, Tower, databases, ORM, auth, reliability, and observability |
| [01 — SwiftCart](01-swiftcart/README.md) | Regional e-commerce marketplace | Multi-tenant order and fulfillment API |
| [02 — PayForge](02-payforge-stripe-clone/README.md) | Stripe-style payment platform | Payment intents, ledger, refunds, and reliable webhooks |
| [03 — BankBridge](03-bankbridge-plaid-clone/README.md) | Plaid-style open-banking platform | Institution linking, account data, transaction sync, and consent |

Add future backend briefs as sibling project directories with their own `README.md`.
