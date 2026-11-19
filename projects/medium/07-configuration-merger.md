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

## Stretch goals

Add nested profiles or live file reload with atomic config replacement.
