# Chapter 1 — Ownership, moves, and cloning

## Challenge 001 — Receipt calculator

**Challenge:** A market stall needs a receipt, but passing its `String` data into helper functions may move it. Build the receipt while deciding which values should move, copy, or be borrowed.

**Your task:** Store a shop name and several `(item, quantity, unit_price_cents)` entries. Write a helper that formats the header, then print every line item, the total, and a closing message that reuses the shop name.

**Requirements:** Use integer cents for money. Make at least one helper accept owned data, observe what becomes unavailable, and then choose a deliberate fix. Explain why numeric values copy while `String` values normally move.

**Example input:** Shop `Green Basket`; apples: `2 × 150` cents; bread: `1 × 250` cents.

**Expected output:**

```text
Green Basket
2 x Apples @ 150 = 300 cents
1 x Bread @ 250 = 250 cents
Total: 550 cents
Thank you for shopping at Green Basket
```

Minor spacing may differ, but the values must match.

**Done when:** The receipt is correct, the shop name is usable in the closing line, and you can identify every move, copy, borrow, or clone in the program.

**Stretch:** Complete the program without cloning the shop name. Then write one sentence explaining when cloning it would still be reasonable.

## Challenge 002 — Contact card handoff

**Challenge:** A formatter takes ownership of contact data, yet the caller still needs the email afterward. Design the ownership handoff explicitly instead of adding clones by habit.

**Your task:** Implement `format_contact(name: String, email: String) -> String`. Call it from `main`, print the card, and then print a follow-up message containing the original email.

**Requirements:** First call the function with the owned strings and observe the compiler error when reusing `email`. Refactor with either a deliberate clone, borrowing, or returning ownership. Explain the tradeoff you chose.

**Example input:** Name `Amina Njeri`, email `amina@example.com`.

**Expected output:**

```text
Amina Njeri <amina@example.com>
Follow-up will be sent to amina@example.com
```

**Done when:** Both lines print and you can trace who owns `name`, `email`, and the formatted result before and after the call.

**Stretch:** Implement two versions: one that clones and one that returns the email with the formatted card. Compare their APIs and allocation costs.

## Challenge 003 — Playlist snapshot

**Challenge:** A queue preview must be edited independently of the original playlist. Decide when moving a collection is enough and when a deep clone is actually required.

**Your task:** Create a `Vec<String>` playlist. Pass it to a length-reporting helper, revise the design so the caller retains the list, then create an editable snapshot and change only the snapshot.

**Requirements:** Show the ownership error caused by consuming the vector. Fix the reporting call without cloning the full list. Clone only when creating the independent snapshot.

**Example input:** Original playlist `['Intro', 'Rustacean', 'Finale']`; remove `Intro` from the preview and add `Encore`.

**Expected output:**

```text
Track count: 3
Original: ["Intro", "Rustacean", "Finale"]
Preview: ["Rustacean", "Finale", "Encore"]
```

**Done when:** Editing the preview does not change the original, and you can explain that cloning `Vec<String>` also clones each owned string.

**Stretch:** Write a helper that consumes a playlist and returns its first track as `Option<String>`.
