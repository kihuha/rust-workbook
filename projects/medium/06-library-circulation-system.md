# Medium 06 — Library circulation system

## Project challenge

Build a small circulation system for books, copies, members, loans, and returns. The challenge is to distinguish a title from its physical copies and enforce loan rules consistently.

## Required behavior

- Store books, individual copy IDs, members, and active loans.
- Search the catalog by title or author.
- Check out an available copy and assign a due date.
- Return a copy and preserve its completed loan history.
- Reject duplicate IDs, unavailable copies, and members at their loan limit.

## Example flow

```text
checkout copy CP-12 to member M-7 on 2026-11-01
-> due 2026-11-15
checkout CP-12 again
-> error: copy already on loan
return CP-12 on 2026-11-10
-> returned 5 days early
```

## Rust skills combined

Domain modeling, structs and enums, maps, date arithmetic, validation, borrowing, persistence, report generation, and tests.

## Done when

The catalog and loan history survive reloads, failed checkouts change nothing, and availability is derived correctly for every copy.

## Stretch goals

Add reservations, overdue fines, or a member borrowing history report.
