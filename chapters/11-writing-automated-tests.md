---
schema: 1
kind: chapter
chapter: 11
slug: writing-automated-tests
title: "Writing Automated Tests"
---

# Chapter 11 — Writing Automated Tests

## Book alignment

Study [Writing Automated Tests](https://rust-book.cs.brown.edu/ch11-00-testing.html): writing tests, controlling test execution, and organizing unit versus integration tests.

## Mastery goals

- Translate requirements into focused assertions.
- Test success, failure, boundaries, and state preservation.
- Use expected panics and `Result`-returning tests appropriately.
- Place tests according to whether they need internals or the public API.

<!-- challenge
id: "11.1"
title: "Test an account library"
status: "not-started"
-->

## Challenge 11.1 — Test an account library

### Brief

Implement a small account library from tests first. Keep balance private and expose deposit, withdrawal, and balance operations.

### Required behavior

- Store money as integer cents.
- Reject zero deposits and withdrawals.
- Reject withdrawals above the balance without changing state.
- Use unit tests for internal validation and integration tests for the public API.

### Expected public API and example flow

```rust
impl Account {
    pub fn new(opening_balance: u64) -> Self;
    pub fn deposit(&mut self, cents: u64) -> Result<(), AccountError>;
    pub fn withdraw(&mut self, cents: u64) -> Result<(), AccountError>;
    pub fn balance(&self) -> u64;
}
```

If you include `main.rs`, keep it as a demonstration: construct the account, execute the documented deposits and withdrawals, and print results through the public API. Tests must call the same methods rather than duplicate account logic.

### Example

```text
opening balance: 1000
deposit: 500 -> balance 1500
withdraw: 400 -> balance 1100
withdraw: 2000 -> error; balance remains 1100
```

### Suggested tests

#### Unit tests

- `T01` — Verify a valid deposit increases balance.
- `T02` — Verify zero deposit returns `InvalidAmount`.
- `T03` — Verify overdraft returns `InsufficientFunds` and preserves balance.
- `T04` — Verify an internal constructor invariant with a custom assertion message.

#### Integration tests

- `T05` — Verify a consumer can complete the documented account flow through public methods only.
- `T06` — Verify private fields cannot be accessed by integration tests.

### Done when

Test names describe behavior, a deliberately introduced bug causes the relevant test to fail, and `cargo test` passes after the fix.

<!-- /challenge -->

<!-- challenge
id: "11.2"
title: "Test-suite organization lab"
status: "not-started"
-->

## Challenge 11.2 — Test-suite organization lab

### Brief

Create a library with fast unit tests, public integration tests, and one expensive test that is ignored by default.

### Required behavior

- Add a shared helper module under `tests/common`.
- Filter tests by a common name fragment.
- Mark one slow test with `#[ignore]` and document how to run it.
- Include one test returning `Result<(), E>` rather than panicking manually.

### Expected library and test flow

Expose one small public parsing function from `src/lib.rs`; keep lower-level parsing helpers private. Unit tests beside the helpers may call private code. Files under `tests/` call only the public function and share setup through `tests/common/mod.rs`. A `main.rs` file is not required for this library-focused challenge.

### Example commands

```text
cargo test validation
cargo test -- --ignored
cargo test -- --show-output
```

### Suggested tests

#### Unit tests

- `T01` — Verify private parsing helpers with valid and invalid inputs.
- `T02` — Verify a `#[should_panic(expected = "...")]` test checks the intended panic message.

#### Integration tests

- `T03` — Verify two integration test files reuse the shared setup helper.
- `T04` — Verify the ignored test does not run under plain `cargo test`.
- `T05` — Verify filtering by `validation` runs only matching tests.

### Done when

You can explain why every test belongs in its chosen location and how to run any subset deliberately.

<!-- /challenge -->
