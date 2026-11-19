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

## Stretch goals

Add priority levels, filter `list` by status, or support a `clear-completed` command.
