# Chapter 2 — Immutable and mutable borrowing

## Challenge 004 — Account preview

**Challenge:** A banking console must display the same account more than once without transferring ownership or cloning its data.

**Your task:** Write `account_summary(label: &str, balance_cents: &i64) -> String`. Call it twice with the same owned label and balance, then use both values again in `main`.

**Requirements:** The helper must only read its inputs. Format cents as a currency value, including two decimal places.

**Example input:** Label `Holiday Fund`, balance `125050` cents.

**Expected output from each call:**

```text
Holiday Fund: 1250.50
```

After both calls, printing the original label and balance must still compile.

**Done when:** No clone is needed and you can explain why multiple shared references are allowed.

**Stretch:** Change `&str` to `&String`, try both a `String` and a string literal as callers, and explain why `&str` is the more flexible parameter.

## Challenge 005 — Inventory restock

**Challenge:** Stock must be updated in place while Rust prevents two writers from changing the quantity at the same time.

**Your task:** Implement `restock(quantity: &mut u32, added: u32)`. Call it twice on the same quantity and print the final stock.

**Requirements:** Also create a temporary failing example with two active mutable references. Record the compiler message, then fix it by ensuring the first borrow ends before the second begins.

**Example input:** Starting quantity `12`; add `5`, then `3`.

**Expected output:**

```text
Final quantity: 20
```

**Done when:** The updates occur through `&mut u32`, and you can state Rust's rule for simultaneous mutable access.

**Stretch:** Change `restock` to use `checked_add` and return an error when the new quantity would exceed `u32::MAX`.

## Challenge 006 — Normalize a name

**Challenge:** An event list contains inconsistent whitespace and capitalization. Normalize each owned name through a mutable borrow so the caller sees the change.

**Your task:** Write `normalize_name(name: &mut String)`. Trim surrounding whitespace and capitalize the first character while lowercasing the remaining characters.

**Requirements:** Update the same `String`; do not make the caller replace it with a returned value. Define empty or whitespace-only input as an empty string.

**Example inputs and results:**

```text
"  aMINA  " -> "Amina"
"KAMAU"     -> "Kamau"
"   "       -> ""
```

Unicode capitalization can produce more than one character. Document whether your first version supports that fully or is intentionally ASCII-only.

**Done when:** The caller's strings contain the normalized values after each call, and the empty case is safe.

**Stretch:** Add `normalized_name(name: &str) -> String` and compare an in-place API with an owned-return API.
