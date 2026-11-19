# Easy 10 — File extension organizer

## Project challenge

Build a safe file organizer that groups files by extension. The challenge is to plan filesystem changes, handle name collisions, and avoid losing data.

## Required behavior

- Scan only the immediate files in a chosen source directory.
- Group known extensions into folders such as `images`, `documents`, and `other`.
- Show a dry-run plan before moving anything.
- Never overwrite an existing destination file.
- Report each successful move and each skipped file.

## Example plan

```text
photo.jpg  -> images/photo.jpg
notes.txt  -> documents/notes.txt
archive    -> other/archive
```

If `images/photo.jpg` already exists, skip the move and report a collision.

## Rust skills combined

Paths, directory iteration, enums, maps, error propagation, ownership, command-line flags, and integration tests using temporary directories.

## Done when

Dry-run makes no changes, collisions never overwrite data, directories are not moved, and one file failure does not hide the final summary.

## System-design foundation

Separate planning from execution, make operations idempotent where possible, and publish changes
atomically. Inject failure between every filesystem step and document recovery. Continue with the
[incremental backup tool](../hard/04-incremental-backup-tool.md).

### Foundation evidence

- `F01` — Inject failure before and after each planned move and prove rerun reaches the intended state without overwriting data.
- `F02` — Property-test generated unsafe names, collisions, symlinks, and traversal attempts inside temporary directories.

## Brown Rust Book prerequisites

Study [module boundaries](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [error propagation](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html), [integration tests](https://rust-book.cs.brown.edu/ch11-03-test-organization.html), and the [file-oriented CLI project](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html).

## Suggested tests

### Unit tests

- `T01` — `.jpg`, `.txt`, and no extension map to the documented categories.
- `T02` — A destination collision produces a skip decision.
- `T03` — Directory entries are excluded from move plans.

### Integration tests

- `T04` — Dry-run against a temporary directory changes nothing.
- `T05` — Execute the example plan and verify all destination paths.
- `T06` — An existing destination file is never overwritten.

## Stretch goals

Add recursive scanning, configurable categories, or a reversible move log.
