# Chapter 20 — Advanced Features

## Book alignment

Study [Advanced Features](https://rust-book.cs.brown.edu/ch20-00-advanced-features.html): unsafe Rust, advanced traits, advanced types, function pointers and closures, and macros.

## Mastery goals

- Encapsulate unsafe operations behind documented safe APIs.
- Use associated types, fully qualified syntax, supertraits, and newtypes.
- Choose type aliases, function pointers, closures, or macros intentionally.
- State safety invariants explicitly.

## Challenge 20.1 — Safe split wrapper over unsafe code

### Brief

Recreate the behavior of splitting one mutable slice into two disjoint mutable slices, isolating pointer operations inside a safe wrapper.

### Required behavior

- Expose a safe `split_at_mut_checked` function.
- Return an error when the index exceeds the slice length.
- Keep the `unsafe` block as small as possible.
- Document why the two returned slices cannot overlap.
- Compare the implementation with the standard library method.

### Example

```text
input: [1, 2, 3, 4], mid: 2
left: [1, 2]
right: [3, 4]
after mutation: [10, 2, 30, 4]
```

### Suggested tests

#### Unit tests

- `T01` — Verify split at zero returns an empty left slice.
- `T02` — Verify split at length returns an empty right slice.
- `T03` — Verify split in the middle returns disjoint expected values.
- `T04` — Verify an out-of-bounds index returns an error before unsafe operations.

#### Integration tests

- `T05` — Verify mutating both outputs changes separate source elements.
- `T06` — Run Miri when available and verify tests report no undefined behavior.

### Done when

The safety comment states all invariants required by each unsafe operation and callers need no unsafe code.

## Challenge 20.2 — Extensible numeric wrapper and macro

### Brief

Build a newtype for millimetres, implement useful advanced traits, and add a small declarative macro for constructing values.

### Required behavior

- Define `Millimeters(u32)` and `Meters(u32)` newtypes.
- Implement addition between supported units with an explicit output type.
- Implement `Display` through a supertrait-based reporting trait.
- Use fully qualified syntax where method names are ambiguous.
- Define `mm![10, 20, 30]` to construct `Vec<Millimeters>`.

### Example

```text
Millimeters(500) + Meters(2) -> Millimeters(2500)
mm![10, 20, 30] -> [Millimeters(10), Millimeters(20), Millimeters(30)]
```

### Suggested tests

#### Unit tests

- `T01` — Verify mixed-unit addition returns `2500 mm`.
- `T02` — Verify checked conversion handles overflow according to policy.
- `T03` — Verify fully qualified syntax selects the intended method.
- `T04` — Verify the macro accepts one and several expressions.

#### Integration tests

- `T05` — Verify consumers can format the newtype but cannot confuse it with raw `u32`.
- `T06` — Add a compile-fail example showing meters and millimetres are not interchangeable without conversion.

### Done when

Each advanced feature solves a stated type-safety or API problem rather than adding abstraction for its own sake.
