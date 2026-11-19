# Easy 01 — Command-line task list

## Project challenge

Build a command-line task list that keeps tasks between program runs. The challenge is to model task state, parse commands, and update a saved collection without mixing those responsibilities together.

## Required behavior

- Support `add`, `list`, `done`, and `remove` commands.
- Give every task a stable numeric ID.
- Store a description and completion state.
- Save tasks to a documented text format after every successful change.
- Treat unknown IDs, missing descriptions, and malformed save data as errors.

## Example session

```text
$ tasks add "Learn ownership"
Added task 1: Learn ownership
$ tasks add "Write tests"
Added task 2: Write tests
$ tasks done 1
Completed task 1
$ tasks list
[x] 1 Learn ownership
[ ] 2 Write tests
```

## Rust skills combined

Structs, vectors, enums, command-line arguments, file I/O, error handling, borrowing, and tests.

## Done when

Tasks survive a restart, IDs do not change when an item is removed, invalid commands do not modify the file, and the domain operations can be tested without launching the CLI.

## System-design foundation

Treat the saved file as a tiny database: define stable task identity, an atomic save boundary, and
what happens after a truncated or incompatible file. This prepares configuration and control-plane
projects where durable desired state must survive restart. Continue with the
[configuration merger](../medium/07-configuration-merger.md).

### Foundation evidence

- `F01` — Interrupt or fail a save before rename and prove the previous complete file still loads.
- `F02` — Load a truncated and an unsupported-version fixture and return a typed error without data loss.

## Brown Rust Book prerequisites

Study [struct methods](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html), [enums and matching](https://rust-book.cs.brown.edu/ch06-00-enums.html), [vectors](https://rust-book.cs.brown.edu/ch08-01-vectors.html), [`Result`](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), [test organization](https://rust-book.cs.brown.edu/ch11-03-test-organization.html), and the [CLI project](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html).

## Suggested tests

### Unit tests

- `T01` — Adding two tasks assigns IDs `1` and `2`.
- `T02` — Completing an unknown ID returns an error and changes nothing.
- `T03` — Removing task `1` does not change task `2`'s ID.

### Integration tests

- `T04` — Add and complete a task through the CLI, restart, and verify its saved state.
- `T05` — Run an invalid command and verify the save file is unchanged.

## Stretch goals

Add priority levels, filter `list` by status, or support a `clear-completed` command.
