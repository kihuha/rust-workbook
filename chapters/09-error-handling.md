# Chapter 9 — Error Handling

## Book alignment

Study [Error Handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html): unrecoverable errors, recoverable `Result` values, propagation with `?`, and guidance on when to panic.

## Mastery goals

- Classify recoverable input failures and broken invariants.
- Preserve useful context while propagating errors.
- Use `panic!`, `unwrap`, and `expect` only when justified.
- Design return types that make failure visible.

## Challenge 09.1 — Validated configuration loader

### Brief

Load a small `key=value` configuration file and distinguish file, syntax, and validation failures.

### Required behavior

- Require `host`, `port`, and `workers` keys.
- Reject duplicate or unknown keys.
- Parse port as `u16` and require workers in `1..=64`.
- Include line numbers in syntax errors.
- Propagate expected failures without `unwrap`.

### Example input

```text
host=127.0.0.1
port=8080
workers=4
```

### Expected result

```text
Config { host: "127.0.0.1", port: 8080, workers: 4 }
```

### Verification checklist

- Verify valid text produces the expected typed configuration.
- Verify `port=eighty` returns an invalid-port error with line `2`.
- Verify missing `workers` returns a missing-key error.
- Verify `workers=0` returns a validation error.


- Verify a missing file is reported separately from invalid contents.
- Verify the CLI exits unsuccessfully and writes errors to standard error.

### Done when

Every expected failure has a distinct, contextual error path and valid configuration reaches the caller.

## Challenge 09.2 — Panic policy audit

### Brief

Implement a seat-reservation type and decide which failures return `Result` and which internal invariant violations may justify a panic.

### Required behavior

- Let callers request a positive number of seats.
- Return errors for zero seats and insufficient capacity.
- Keep internal reserved seats at or below capacity.
- Document any `assert!`, `expect`, or unreachable state with its invariant.

### Example

```text
capacity: 10
reserve 4 -> Ok; remaining 6
reserve 7 -> Err(InsufficientCapacity { requested: 7, available: 6 })
```

### Verification checklist

- Verify reserving four of ten seats leaves six.
- Verify an oversized request returns an error and leaves capacity unchanged.
- Verify zero seats is rejected.
- Verify the invariant `reserved <= capacity` after every public operation.


- Verify a sequence of accepted and rejected requests produces the correct final state.
- If an invariant panic exists, verify only deliberately corrupted internal test state can trigger it.

### Done when

Caller mistakes are recoverable and panics are limited to states the public API should make impossible.
