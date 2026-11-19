# Chapter 11 — Smart pointers and interior mutability

## Challenge 031 — Shared document outline

**Challenge:** Several panels need shared ownership of one immutable outline without duplicating its contents.

**Your task:** Store a `DocumentOutline` in `Rc<T>`. Give an `Rc` clone to navigation and search panels, then drop owners while recording `Rc::strong_count`.

**Expected count sequence:**

```text
document only: 1
document + navigation: 2
document + navigation + search: 3
after search is dropped: 2
```

**Requirements:** Print the same outline title through each owner. Do not clone the `DocumentOutline` itself.

**Done when:** The counts match and you can explain that `Rc::clone` increments shared ownership rather than copying the stored value.

**Stretch:** Give a panel a `Weak<DocumentOutline>` reference and show that `upgrade()` returns `None` after all strong owners are dropped.

## Challenge 032 — Shared visit counter

**Challenge:** Multiple single-threaded components must mutate one shared counter even though they hold shared owners.

**Your task:** Share `Rc<RefCell<u32>>` between two components. Let one add `2` visits and the other add `3`, then read the final count.

**Expected output:**

```text
Final visits: 5
```

**Requirements:** Keep each `borrow_mut` scope as short as possible. In a separate test marked `#[should_panic]`, hold one mutable borrow and request another to observe runtime enforcement.

**Done when:** Normal updates succeed and you can explain why overlapping mutable borrows compile but panic at runtime with `RefCell`.

**Stretch:** Redesign the example so one owner coordinates updates through ordinary `&mut` access, then compare the APIs.

## Challenge 033 — Shared tree nodes

**Challenge:** A child should navigate to its parent without creating an `Rc` cycle that leaks the tree.

**Your task:** Define a node with owned `Rc` children and a `Weak` parent stored behind `RefCell`. Build `Grandparent -> Parent -> Child` and traverse both directions.

**Expected ownership behavior:**

```text
child.parent.upgrade() -> Some(parent)
after all strong parent owners are dropped:
child.parent.upgrade() -> None
```

**Requirements:** Parent-to-child links must be strong; child-to-parent links must be weak. Print relevant strong and weak counts before and after dropping the root.

**Done when:** Bidirectional navigation works while weak parent links do not keep ancestors alive.

**Stretch:** Implement `Drop` for the node and record destruction order.
