# Chapter 6 — Collections, strings, and slices

## Challenge 016 — Daily expense summary

**Mini brief:** Track a day's purchases and report the total, largest item, and number of purchases above a chosen threshold.

**Practice:** `Vec`, iteration, borrowing collection elements, and safe empty-collection handling.

**Requirements:** Store expenses as a vector of `(description, cents)` pairs. Write summary helpers that do not consume the vector. Define what the largest-item result should be when there are no expenses.

**Done when:** Empty, one-item, and multi-item cases are handled without indexing panics.

**Stretch:** Sort a separate view of the expenses by amount while preserving the original order.

## Challenge 017 — Word statistics

**Mini brief:** A study tool reads a sentence and reports its word count, longest word, and average word length.

**Practice:** `&str`, `String`, slices, Unicode caveats, and iterators over text.

**Requirements:** Accept a `&str`, split on whitespace, and compute the requested statistics. Return an explicit empty-input result rather than dividing by zero. Do not assume a string can be indexed like a byte array of characters.

**Done when:** Multiple spaces and empty input behave sensibly.

**Stretch:** Count Unicode scalar values in each word and document how that differs from user-perceived grapheme clusters.

## Challenge 018 — Frequency table

**Mini brief:** Count how often each word appears in a short passage and display the most frequent words first.

**Practice:** `HashMap`, owned keys, entry APIs, and sorting borrowed map entries for display.

**Requirements:** Normalize words to lowercase, count them, and print a stable descending frequency report. Keep tie ordering deterministic.

**Done when:** Repeated words count correctly and equal-frequency output is predictable.

**Stretch:** Ignore punctuation without losing apostrophes inside contractions.
