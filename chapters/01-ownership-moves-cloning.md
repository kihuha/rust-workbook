# Chapter 1 — Ownership, moves, and cloning

## Challenge 001 — Receipt calculator

**Mini brief:** A market stall needs a command-line receipt for a few fixed items. Build a small program that stores a shop name, item names, quantities, and unit prices, then prints a readable total.

**Practice:** Scalar types, `String`, assignment, formatting, and the difference between `Copy` values and moved `String` values.

**Requirements:** Make a helper that formats a receipt header from a shop name. Call it, then decide whether the name can still be used afterward. Resolve any ownership issue deliberately and explain why your fix works.

**Done when:** The output includes each line item and a total, and your notes identify which values copied and which moved.

**Stretch:** Reuse the shop name in a closing message without cloning unless there is a good reason.

## Challenge 002 — Contact card handoff

**Mini brief:** A tiny contact-card tool accepts a person's name and email, then hands the card to a formatter that returns one display string.

**Practice:** Moving owned values into functions, returning ownership, and choosing when a clone is justified.

**Requirements:** Implement `format_contact(name: String, email: String) -> String`. In `main`, try using both inputs after the call. Refactor so the program can print the formatted card and still retain the email for a follow-up line, making the ownership choice explicit.

**Done when:** It prints both lines and you can describe where the owned strings move.

**Stretch:** Compare a clone-based solution with a solution that returns the original data in a tuple.

## Challenge 003 — Playlist snapshot

**Mini brief:** A music app keeps a short playlist and needs a separately editable snapshot for a “queue preview.”

**Practice:** `Vec<String>`, moves of collections, and deep cloning when independent data is required.

**Requirements:** Create a playlist, transfer it to a function that reports its length, then revise the design so the caller can keep using the playlist. Make a separate snapshot, edit one list, and show that the other remains unchanged.

**Done when:** The output demonstrates independent edits, and your notes say what `clone` copies here.

**Stretch:** Add a helper that consumes a playlist and returns only its first track, handling an empty playlist.
