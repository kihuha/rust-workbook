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

## Stretch goals

Add recursive scanning, configurable categories, or a reversible move log.
