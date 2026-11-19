# Easy 04 — Contact book

## Project challenge

Build a contact book that supports searching and updating saved records. The challenge is to choose stable identity rules instead of treating display names as unique.

## Required behavior

- Store an ID, name, phone number, and email address.
- Support `add`, `list`, `find`, `update`, and `remove`.
- Search names case-insensitively and allow multiple matches.
- Reject blank names and duplicate non-empty email addresses.
- Persist contacts in a documented format.

## Example session

```text
$ contacts add "Amina Njeri" 0712345678 amina@example.com
Added contact 1
$ contacts find amina
1 | Amina Njeri | 0712345678 | amina@example.com
$ contacts update 1 phone 0799999999
Updated contact 1
```

## Rust skills combined

Structs, vectors or maps, enums for update fields, string normalization, borrowing, file I/O, errors, and tests.

## Done when

Updates target IDs, ambiguous name searches return every match, and invalid changes leave both memory and saved data unchanged.

## Stretch goals

Import and export contacts as CSV or add tags.
