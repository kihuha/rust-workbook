# Hard 05 — Expression language interpreter

## Project challenge

Build an interpreter for a small dynamically evaluated language. The challenge is to turn source text into tokens, syntax trees, scoped values, and useful diagnostics.

## Required behavior

- Tokenize numbers, booleans, strings, identifiers, operators, delimiters, and keywords.
- Parse precedence-aware expressions, variable declarations, blocks, conditionals, and function calls.
- Evaluate with lexical scopes and explicit runtime value types.
- Report source spans for lexical, parse, and runtime errors.
- Provide a REPL and execute source files.

## Example program

```text
let price = 1250;
let quantity = 3;
if quantity > 2 {
  print(price * quantity);
}
```

**Expected output:** `3750`.

## Architecture expectations

Use distinct token, AST, environment, value, and diagnostic types. Keep parsing separate from evaluation. Avoid panics for user-authored invalid programs.

## Rust skills combined

Enums, recursive data, parsers, source spans, scoped environments, ownership, error recovery, traits, REPL I/O, and language-level tests.

## Done when

Operator precedence is tested, shadowing follows documented rules, errors point to relevant source text, and the REPL continues after a recoverable error.

## Stretch goals

Add user-defined functions, closures, lists, or a bytecode compiler.
