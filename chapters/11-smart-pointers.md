# Chapter 11 — Smart pointers and interior mutability

## Challenge 031 — Shared document outline

**Mini brief:** Several UI panels need to refer to the same immutable document outline.

**Practice:** `Rc<T>` for shared ownership in a single-threaded program.

**Requirements:** Store the outline in an `Rc`, give clones to several panels, and observe the strong reference count as owners are added and dropped.

**Done when:** You can explain why cloning the `Rc` is different from cloning the underlying outline.

**Stretch:** Add a `Weak` back-reference and show that it does not keep the outline alive.

## Challenge 032 — Shared visit counter

**Mini brief:** Multiple components in a single-threaded simulation update a shared count of viewed pages.

**Practice:** `RefCell<T>`, runtime borrow checking, and interior mutability.

**Requirements:** Share a counter through `Rc<RefCell<u32>>`, update it from two components, and read the final value. In a separate experiment, trigger an overlapping mutable borrow and record the runtime behavior.

**Done when:** You can explain when `RefCell` moves borrow checking from compile time to runtime.

**Stretch:** Replace the counter with a design that avoids interior mutability and compare it.

## Challenge 033 — Shared tree nodes

**Mini brief:** Build a small family tree where a child can refer back to a parent without creating a reference-counting cycle.

**Practice:** Combining `Rc`, `RefCell`, and `Weak`.

**Requirements:** Model parent-to-child ownership and a weak child-to-parent link. Demonstrate that dropping the root allows the tree nodes to be released.

**Done when:** The structure can be traversed in both directions while parent links do not keep the whole tree alive.

**Stretch:** Track a simple drop notification to observe destruction order.
