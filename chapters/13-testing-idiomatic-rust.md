# Chapter 13 — Testing and idiomatic Rust

## Challenge 037 — Test the account rules

**Challenge:** Turn banking requirements into tests that define both successful behavior and rejected boundary cases.

**Your task:** Test deposit, withdrawal, and transfer operations with unit tests for local rules and at least one integration test through the public API.

**Required cases:**

```text
deposit 500 into 1000        -> balance 1500
withdraw 400 from 1000       -> balance 600
withdraw 1200 from 1000      -> InsufficientFunds; balance 1000
deposit or withdraw 0        -> InvalidAmount
transfer 300 between accounts -> balances change by -300 and +300
```

**Requirements:** Verify errors and unchanged state after failed operations. Use setup helpers only when they make the rule easier to read. Give tests behavior-focused names.

**Done when:** `cargo test` covers all required cases and failures clearly identify the broken rule.

**Stretch:** Loop over a range of valid transfer amounts and assert that the combined balance of both accounts never changes.

## Challenge 038 — Refactor for idioms

**Challenge:** Improve awkward but working Rust while proving that its observable behavior stays the same.

**Your task:** Choose an earlier completed challenge. Add characterization tests, run Clippy, and make at least three justified improvements.

**Required changes:** Remove one needless clone, replace one manual pattern with an iterator or standard method, and introduce one clearer type or function boundary.

**Example preserved behavior:**

```text
before: summary([150, 800]) -> "Total: 950 cents"
after:  summary([150, 800]) -> "Total: 950 cents"
```

**Requirements:** Do not accept Clippy suggestions blindly. Record each warning, the change made or rejected, and the reason. Keep formatting and tests passing.

**Done when:** `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` pass with identical behavior.

**Stretch:** Add a public function example that runs successfully as a doctest.

## Challenge 039 — Capstone: personal ledger

**Challenge:** Combine the workbook's core ideas into a durable ledger whose domain logic is independent from terminal and file I/O.

**Your task:** Build a CLI that records deposits and withdrawals, saves them, loads them later, and reports total deposits, total withdrawals, and current balance.

**Example commands:**

```text
ledger add deposit 250000 "Salary"
ledger add withdrawal 80000 "Rent"
ledger summary
```

**Expected summary:**

```text
Deposits: 250000 cents
Withdrawals: 80000 cents
Balance: 170000 cents
```

**Requirements:** Store money as integer cents. Reject zero amounts and withdrawals above the available balance. Document the saved text format and report malformed lines with line numbers.

Keep parsing, ledger operations, reporting, and persistence in separate modules. Core operations must be testable without terminal input or real files.

**Done when:** Data survives a save/load round trip, invalid operations leave state unchanged, errors do not panic, and unit plus integration tests cover the main flow.

**Stretch:** After the synchronous core is reliable, add either a concurrent read-only report or an async remote backup behind a separate interface.
