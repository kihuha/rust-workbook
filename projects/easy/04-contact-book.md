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

## Brown Rust Book prerequisites

Study [borrowing](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html), [methods](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html), [vectors and maps](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [error design](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), and [automated tests](https://rust-book.cs.brown.edu/ch11-00-testing.html).

## Suggested tests

### Unit tests

- `T01` — Adding a valid contact assigns a stable ID.
- `T02` — Case-insensitive search returns all matching names.
- `T03` — A duplicate non-empty email is rejected.
- `T04` — A failed update preserves the original contact.

### Integration tests

- `T05` — Add, update, save, reload, and find the example contact.
- `T06` — Removing one contact does not renumber the others.

## Stretch goals

Import and export contacts as CSV or add tags.
