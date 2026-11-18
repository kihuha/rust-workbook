# Chapter 2 — Immutable and mutable borrowing

## Challenge 004 — Account preview

**Mini brief:** A banking console should display an account summary without taking ownership of the account name or balance data.

**Practice:** Shared references and read-only function parameters.

**Requirements:** Write a function that accepts `&str` for the account label and `&f64` (or an integer cents value) for the balance, then returns a summary. Call it more than once with the same data.

**Done when:** The caller still owns and can use its values after each call. Explain why the function needs no clone.

**Stretch:** Change the label argument to `&String`, then decide which signature is more flexible and why.

## Challenge 005 — Inventory restock

**Mini brief:** A shop worker needs to increase the quantity of an item in place and then inspect the updated stock.

**Practice:** Mutable references, `&mut`, and one-writer borrowing rules.

**Requirements:** Implement `restock(quantity: &mut u32, added: u32)`. Call it twice and print the result. Try to create a second active mutable reference and record the compiler's complaint before fixing the code.

**Done when:** Restocking works, and you can state the rule Rust enforces for simultaneous mutable access.

**Stretch:** Reject a restock that would overflow the quantity using `checked_add`.

## Challenge 006 — Normalize a name

**Mini brief:** An event sign-in list should remove accidental surrounding whitespace and capitalize attendee names.

**Practice:** Mutating owned `String` values through a mutable borrow; distinguishing a borrow from an owned return value.

**Requirements:** Write a function that takes `&mut String`, trims surrounding whitespace, and updates the same string. Run it on several names and verify the caller sees each edit.

**Done when:** The function changes the caller's string without returning a new `String`.

**Stretch:** Add a second helper that instead takes `&str` and returns a new normalized `String`; compare the APIs.
