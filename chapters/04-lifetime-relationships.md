# Chapter 4 — Lifetime relationships

These briefs focus first on whether references remain valid. They do not require explicit lifetime annotations; introduce signature annotations only when borrowing and validity feel comfortable.

## Challenge 010 — Choose the longer-lived input

**Mini brief:** A report helper receives a title and a subtitle and should return one of them as a borrowed label.

**Practice:** Reasoning about multiple input references and why a returned reference must point to valid data.

**Requirements:** Sketch a function that returns the longer label by character count. Before coding, write down what must be true about both input values while the returned label is used. Implement it with Rust's required signature syntax, then test which inputs can outlive the result.

**Done when:** You can explain why the result cannot refer to a temporary local string created inside the function.

**Stretch:** Return an owned `String` instead and compare the caller's obligations.

## Challenge 011 — Catalog search result

**Mini brief:** A catalog has a list of borrowed book titles. A search should return a matching title without allocating a new string.

**Practice:** Lifetimes of collection elements and returned slices/references.

**Requirements:** Store titles in a `Vec<String>`. Search by prefix and return a borrowed match if one exists. Keep the catalog alive while displaying the result; then reorganize the scopes and identify which arrangement is invalid.

**Done when:** The successful lookup borrows from the catalog and the missing case is represented safely.

**Stretch:** Make the lookup case-insensitive while still returning the original title.

## Challenge 012 — Borrow or own?

**Mini brief:** A command parser receives either a word already present in the input line or a generated default command.

**Practice:** Choosing between borrowed and owned results when data may come from different places.

**Requirements:** Parse the first token from an input line. If it is absent, use a default command. Design the result so both cases can be represented without returning a reference to a local temporary. Describe the ownership and lifetime tradeoff before implementing.

**Done when:** Empty and non-empty input both work without dangling references.

**Stretch:** Try `Cow<'a, str>` after you have studied its purpose; explain when it avoids allocation.
