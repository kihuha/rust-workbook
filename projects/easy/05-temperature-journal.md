# Easy 05 — Temperature journal

## Project challenge

Build a weather journal that records daily temperatures and produces summaries. The challenge is to validate measurements and derive statistics from optional data.

## Required behavior

- Record a date and Celsius temperature.
- Replace an existing entry for the same date only after explicit confirmation or a `--replace` flag.
- Report minimum, maximum, and mean temperature.
- Convert displayed values between Celsius and Fahrenheit.
- Save and load the journal from a text file.

## Example data and summary

```text
2026-11-01: 24.0 C
2026-11-02: 27.5 C
2026-11-03: 23.5 C

minimum: 23.5 C
maximum: 27.5 C
mean: 25.0 C
```

## Rust skills combined

Structs, floating-point values, validation, collections, iterators, `Option`, file I/O, parsing, and tests.

## Done when

Empty journals return no statistics, duplicate dates follow the replacement rule, and conversion tests allow a small floating-point tolerance.

## Brown Rust Book prerequisites

Study [data types and control flow](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html), [structs](https://rust-book.cs.brown.edu/ch05-00-structs.html), [vectors](https://rust-book.cs.brown.edu/ch08-01-vectors.html), [writing tests](https://rust-book.cs.brown.edu/ch11-01-writing-tests.html), and [iterator adapters](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — The example minimum is `23.5°C`, maximum `27.5°C`, and mean `25.0°C`.
- `T02` — An empty journal returns no statistics.
- `T03` — Celsius and Fahrenheit round-trip within the documented tolerance.
- `T04` — A duplicate date without replacement permission is rejected.

### Integration tests

- `T05` — Save and reload the example observations without changing values.
- `T06` — The `--replace` flow updates exactly one existing date.

## Stretch goals

Show the hottest streak or render a simple text chart.
