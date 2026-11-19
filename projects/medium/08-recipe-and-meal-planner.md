# Medium 08 — Recipe and meal planner

## Project challenge

Build a planner that stores recipes, schedules meals, and combines ingredient quantities into a shopping list. The challenge is to normalize units without silently combining incompatible measurements.

## Required behavior

- Store recipes with servings and measured ingredients.
- Schedule recipes for dates and desired serving counts.
- Scale ingredient amounts for each planned meal.
- Combine matching ingredients only when their units are compatible.
- Save recipes and the weekly plan separately.

## Example calculation

```text
Pancakes for 2 uses 250 ml milk and 200 g flour.
Schedule Pancakes for 4 twice.

shopping list:
milk: 1000 ml
flour: 800 g
```

## Rust skills combined

Domain types, enums for units, decimal calculations, maps, iterators, serialization, validation, modules, and tests.

## Done when

Scaling is exact enough for documented units, incompatible units remain separate, missing recipes fail clearly, and saving a plan does not duplicate recipes.

## Brown Rust Book prerequisites

Study [structs](https://rust-book.cs.brown.edu/ch05-00-structs.html), [data-bearing enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [maps](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html), [generic behavior](https://rust-book.cs.brown.edu/ch10-00-generics.html), and [iterator aggregation](https://rust-book.cs.brown.edu/ch13-02-iterators.html).

## Suggested tests

### Unit tests

- `T01` — Scaling two servings to four doubles every amount.
- `T02` — The example plan totals `1000 ml` milk and `800 g` flour.
- `T03` — Incompatible units remain separate.
- `T04` — Missing recipes return an error before changing the plan.

### Integration tests

- `T05` — Save and reload recipes separately from a weekly plan.
- `T06` — A generated shopping list is deterministic across runs.

## Stretch goals

Convert compatible units, track pantry stock, or optimize a shopping list by store section.
