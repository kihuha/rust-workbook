---
schema: 1
kind: chapter
chapter: 15
slug: smart-pointers
title: "Smart Pointers"
---

# Chapter 15 — Smart Pointers

## Book alignment

Study [Smart Pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html): `Box`, `Deref`, `Drop`, `Rc`, `RefCell`, and preventing reference cycles with `Weak`.

## Mastery goals

- Use heap indirection for recursive types.
- Understand deref coercion and deterministic cleanup.
- Model shared ownership with reference counts.
- Apply interior mutability carefully and avoid cycles.

<!-- challenge
id: "15.1"
title: "Shared document tree"
status: "not-started"
-->

## Challenge 15.1 — Shared document tree

### Brief

Build a tree in which parents own children and children can navigate to parents without creating a reference cycle.

### Required behavior

- Store children as `Vec<Rc<RefCell<Node>>>`.
- Store the parent as `Weak<RefCell<Node>>`.
- Add children through a method that updates both directions.
- Print strong and weak counts before and after dropping the root.

### Expected interface and `main.rs` flow

```rust
impl Node {
    fn new(name: impl Into<String>) -> Rc<RefCell<Self>>;
    fn add_child(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>);
    fn parent(&self) -> Option<Rc<RefCell<Node>>>;
}
```

`main` creates `root`, `chapter`, and `section`, connects them through `add_child`, and uses `parent` for upward navigation. It prints reference counts before and after parent owners leave scope. Callers must not set parent and child links separately.

### Example

```text
root -> chapter -> section
section parent: chapter
after root and chapter owners drop:
section parent: unavailable
```

### Suggested tests

#### Unit tests

- `T01` — Verify adding a child creates a strong parent-to-child link.
- `T02` — Verify the child’s weak parent upgrades while the parent lives.
- `T03` — Verify dropping strong parent owners makes `upgrade()` return `None`.
- `T04` — Verify a node can be renamed through controlled interior mutability.

#### Integration tests

- `T05` — Verify traversing down and back up produces the expected names.
- `T06` — Verify `Drop` observations show nodes are released rather than leaked in a cycle.

### Done when

Reference counts match your predictions and weak back-references do not keep ancestors alive.

<!-- /challenge -->

<!-- challenge
id: "15.2"
title: "Recursive expression with `Box`"
status: "not-started"
-->

## Challenge 15.2 — Recursive expression with `Box`

### Brief

Represent and evaluate an arithmetic expression tree whose recursive variants have a known size through indirection.

### Required behavior

- Define literal, addition, multiplication, and negation variants.
- Use `Box<Expr>` for recursive children.
- Implement evaluation and formatted display.
- Add a wrapper type that logs cleanup through `Drop` for observation only.

### Expected interface and `main.rs` flow

```rust
impl Expr {
    fn evaluate(&self) -> i64;
}

impl std::fmt::Display for Expr { /* ... */ }
```

`main` constructs the documented tree with boxed child expressions, prints it through `Display`, and prints `evaluate()`. The cleanup wrapper may log when dropped, but expression evaluation must not depend on those messages.

### Example

```text
expression: (2 + 3) * -4
display: ((2 + 3) * (-4))
result: -20
```

### Suggested tests

#### Unit tests

- `T01` — Verify a literal evaluates to itself.
- `T02` — Verify nested addition and multiplication obey tree structure.
- `T03` — Verify negation works for a nested expression.
- `T04` — Verify display includes unambiguous parentheses.

#### Integration tests

- `T05` — Verify a constructed tree evaluates and displays as documented.
- `T06` — Verify cleanup messages occur when the owning expression leaves scope.

### Done when

You can explain why recursive children require indirection and why `Box` does not imply shared ownership.

<!-- /challenge -->
