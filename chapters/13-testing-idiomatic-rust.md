# Chapter 13 — Testing and idiomatic Rust

## Challenge 037 — Test the account rules

**Mini brief:** Before releasing the banking prototype, lock down its deposit, withdrawal, and transfer rules.

**Practice:** Unit tests, integration tests, test fixtures, and boundary cases.

**Requirements:** Add tests for successful operations, insufficient funds, zero amounts, and balance boundaries. Keep tests readable and avoid duplicating setup unnecessarily.

**Done when:** `cargo test` covers success and failure behavior and each test name describes the rule it checks.

**Stretch:** Add a property-style test loop that checks a useful invariant over a range of amounts.

## Challenge 038 — Refactor for idioms

**Mini brief:** A teammate wrote a working but awkward Rust report generator. Improve it without changing output.

**Practice:** Clippy feedback, iterators, meaningful types, and avoiding needless clones.

**Requirements:** Start from one of your earlier challenges. Run `cargo clippy`, fix useful warnings, replace at least one unnecessary clone, and preserve behavior with tests.

**Done when:** The diff is behavior-preserving and you can explain each change you kept.

**Stretch:** Add documentation examples for one public function and ensure they compile as doctests.

## Challenge 039 — Capstone: personal ledger

**Mini brief:** Build a small ledger application that records transactions, supports deposits and withdrawals, and produces a summary from saved data.

**Practice:** Integrating ownership, structs/enums, collections, error handling, modules, iterators, and tests.

**Requirements:** Model money in integer cents. Validate user input. Keep ledger operations testable apart from terminal input/output. Load and save a documented text format. Report total deposits, withdrawals, and current balance.

**Done when:** The app handles malformed data and invalid operations without panicking, has unit and integration tests, and its modules have clear responsibilities.

**Stretch:** Add a concurrent read-only report or an async remote backup after the core program is reliable.
