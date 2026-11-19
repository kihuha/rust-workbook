# Medium 07 — Configuration merger

## Project challenge

Build a configuration loader that merges defaults, a file, environment values, and command-line overrides. The challenge is to make precedence and validation transparent.

## Required behavior

- Define a typed application configuration with required and optional fields.
- Apply precedence: defaults < file < environment < CLI.
- Convert strings into typed ports, durations, booleans, and paths.
- Report all validation problems together where practical.
- Print the final configuration with secret values redacted.

## Example sources

```text
default port: 8080
file port: 9000
environment port: 7000
CLI port: 6000
final port: 6000
```

## Architecture expectations

Keep raw source values separate from the validated config. Track the winning source for each field so diagnostics can explain where a value came from.

## Rust skills combined

Serde-style decoding, enums, generic parsing helpers, environment and CLI access, validation, error aggregation, redaction, and tests.

## Done when

Every precedence combination is tested, invalid values name their source, secrets never appear in debug output, and the final type cannot represent missing required fields.

## Brown Rust Book prerequisites

Study [enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [error design](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [generics and traits](https://rust-book.cs.brown.edu/ch10-00-generics.html), and [arguments plus environment variables](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html).

## Suggested tests

### Unit tests

- `T01` — CLI wins over environment, file, and default values.
- `T02` — Every typed parser reports the source of an invalid value.
- `T03` — Missing required fields prevent final configuration construction.
- `T04` — Debug and display output redact secrets.

### Integration tests

- `T05` — The example sources produce final port `6000`.
- `T06` — Environment and CLI inputs override a real temporary config file.

## Stretch goals

Add nested profiles or live file reload with atomic config replacement.
