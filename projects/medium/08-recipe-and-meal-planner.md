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

## Stretch goals

Convert compatible units, track pantry stock, or optimize a shopping list by store section.
