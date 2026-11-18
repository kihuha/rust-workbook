# Chapter 8 — Error handling

## Challenge 022 — Validated deposit

**Mini brief:** A banking terminal accepts a deposit amount as text and must distinguish malformed input from a non-positive amount.

**Practice:** `Result`, custom error enums, and parsing.

**Requirements:** Parse a whole number of cents, reject zero and negative values, and return descriptive errors. Keep input acquisition separate from validation.

**Done when:** Valid input produces a deposit amount and each invalid category yields a distinct message.

**Stretch:** Implement `Display` for the custom error.

## Challenge 023 — CSV expense loader

**Mini brief:** Load a small comma-separated expense file and report which row failed if its amount is invalid.

**Practice:** File I/O, `?`, error propagation, and adding context to failures.

**Requirements:** Read each line as `description,amount_cents`. Return a collection on success, or an error that identifies the line number and underlying problem. Handle a missing file cleanly.

**Done when:** Valid rows load, malformed rows identify their location, and no `unwrap` is used for expected failures.

**Stretch:** Support descriptions containing commas by choosing and documenting a simple quoting rule or a CSV crate.

## Challenge 024 — Safe calculator

**Mini brief:** A command-line calculator supports addition, subtraction, multiplication, and integer division.

**Practice:** `Option`/`Result`, division-by-zero handling, and clear error types.

**Requirements:** Parse two signed integers and an operator. Report malformed operands, unknown operators, and division by zero distinctly. Use checked arithmetic for overflow.

**Done when:** Every failure path returns a useful error instead of panicking.

**Stretch:** Add remainder and document behavior for negative operands.
