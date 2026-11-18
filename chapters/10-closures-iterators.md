# Chapter 10 — Closures and iterators

## Challenge 028 — Filtered score report

**Mini brief:** A course dashboard needs the mean score for passing students and a count of students who need support.

**Practice:** Iterator adapters, closures, and `filter`/`map`/`sum`.

**Requirements:** Given a slice of scores, compute both results without an explicit indexing loop. Represent the no-passing-students case clearly.

**Done when:** Empty input and all-failing input do not cause division by zero.

**Stretch:** Compare the iterator version with a loop version for readability.

## Challenge 029 — Transaction pipeline

**Mini brief:** A ledger report should select deposits from a date range, convert them into display rows, and total their values.

**Practice:** Lazy iterator pipelines, borrowing items, and collecting only when useful.

**Requirements:** Define a transaction type, filter a borrowed slice by date and kind, then compute a total without cloning transactions.

**Done when:** The pipeline stays readable and the original ledger remains available.

**Stretch:** Split the pipeline into named iterator helpers and compare the resulting types/API.

## Challenge 030 — Custom sequence iterator

**Mini brief:** A reading app displays page numbers in a user-selected interval.

**Practice:** Implementing `Iterator`, `next`, and termination behavior.

**Requirements:** Create a `PageRange` iterator that yields page numbers from start through end. Ensure invalid or empty ranges yield no items.

**Done when:** It works in a `for` loop and stops after the last page.

**Stretch:** Implement `size_hint` and explain what it promises.
