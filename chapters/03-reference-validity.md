# Chapter 3 — Reference validity, scopes, and non-lexical lifetimes

## Challenge 007 — Borrow timeline notebook

**Mini brief:** A teammate keeps getting borrow-checker errors while updating a score. Build a tiny program and annotate when each reference is actually used.

**Practice:** Reference validity and non-lexical lifetimes (NLL).

**Requirements:** Start with an integer score. Create an immutable reference, print through it, then update the score and print the owner. Add a second immutable reference used later. For each version, draw a simple owner/reference timeline and explain whether the update is allowed.

**Done when:** You can explain why a borrow can end at its last use even if its lexical scope continues.

**Stretch:** Put the read-only work in an inner block and compare the timeline with NLL.

## Challenge 008 — Safe first word

**Mini brief:** A log viewer needs to display the first word of a line without copying the entire line.

**Practice:** String slices as borrowed views and keeping a reference tied to valid source data.

**Requirements:** Implement `first_word(line: &str) -> &str`. Return the prefix before the first space, or the whole input when there is no space. Keep the source string alive while using the result.

**Done when:** Empty input, one-word input, and multi-word input work, and you can point to the data the returned slice refers to.

**Stretch:** Add punctuation as a delimiter while keeping the function Unicode-safe at character boundaries.

## Challenge 009 — Edit after inspection

**Mini brief:** A note editor previews a note, then appends a status marker after the preview is finished.

**Practice:** The end of immutable borrows, mutation after last use, and scoping references clearly.

**Requirements:** Borrow a `String` to compute and print a preview. After its final use, append `" [saved]"` to the original. Then make a second version where the preview is used after the append attempt; use the compiler result to explain the difference.

**Done when:** Both versions make the borrow timeline visible and compile behavior is explained in your own words.

**Stretch:** Refactor the preview into a helper that returns an owned short summary, then compare tradeoffs.
