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

## System-design foundation

Treat the language as a versioned public contract: bound parse/evaluation work, preserve source
diagnostics, and add compatibility fixtures before extending syntax. Policy engines and query
languages need the same safety and evolution discipline.

### Foundation evidence

- `F01` — Fuzz/token-generate bounded programs and assert invalid input returns diagnostics without panic or unbounded recursion/work.
- `F02` — Run old/new syntax fixtures through a version change and document accepted, migrated, and explicitly rejected forms.

## Brown Rust Book prerequisites

Study [enums and matching](https://rust-book.cs.brown.edu/ch06-00-enums.html), [traits and lifetimes](https://rust-book.cs.brown.edu/ch10-00-generics.html), [boxed recursive types](https://rust-book.cs.brown.edu/ch15-01-box.html), and [patterns](https://rust-book.cs.brown.edu/ch19-00-patterns.html).

## Suggested tests

### Unit tests

- `T01` — Tokenize every supported token and retain source spans.
- `T02` — Parse multiplication with higher precedence than addition.
- `T03` — Lexical scope shadowing restores the outer binding afterward.
- `T04` — Type errors identify the responsible expression span.

### Integration tests

- `T05` — The example program prints `3750`.
- `T06` — A source file with a syntax error exits with a contextual diagnostic.
- `T07` — The REPL accepts another expression after a recoverable error.

## Stretch goals

Add user-defined functions, closures, lists, or a bytecode compiler.
