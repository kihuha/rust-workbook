---
schema: 1
kind: chapter
chapter: 5
slug: using-structs
title: "Using Structs to Structure Related Data"
---

# Chapter 5 — Using Structs to Structure Related Data

## Book alignment

Study [Using Structs to Structure Related Data](https://rust-book.cs.brown.edu/ch05-00-structs.html): defining structs, the rectangle example, methods, and associated functions.

## Mastery goals

- Group related values into meaningful types.
- Use field init shorthand and struct update syntax appropriately.
- Define methods that borrow or mutate `self`.
- Use associated functions as constructors.

<!-- challenge
id: "05.1"
title: "Library book record"
status: "not-started"
-->

## Challenge 05.1 — Library book record

### Brief

Create a `Book` type that owns its descriptive data and keeps borrowing state consistent through methods.

### Required behavior

- Store an ISBN, title, author, and availability state.
- Provide `Book::new`, `summary`, `checkout`, and `return_book`.
- Reject checkout when already borrowed.
- Reject return when already available.

### Expected interface and `main.rs` flow

```rust
impl Book {
    fn new(isbn: String, title: String, author: String) -> Self;
    fn summary(&self) -> String;
    fn checkout(&mut self) -> Result<(), String>;
    fn return_book(&mut self) -> Result<(), String>;
}
```

`main` constructs a book with `Book::new`, prints `summary`, performs the two checkout attempts and one return, then prints each result. Only these methods may change availability.

### Example

```text
978-1 | The Rust Book by Rust Community | available
checkout -> success
checkout -> error: already borrowed
return -> success
```

### Verification checklist

- Verify `new` creates an available book with the supplied fields.
- Verify checkout changes availability exactly once.
- Verify a second checkout returns an error without changing state.
- Verify returning an available book returns an error.


- Verify the complete checkout and return session prints the expected summaries.
- Verify two books keep independent state.

### Done when

State changes occur only through methods and the struct cannot accidentally mix data belonging to different books.

<!-- /challenge -->

<!-- challenge
id: "05.2"
title: "Rectangle packing report"
status: "not-started"
-->

## Challenge 05.2 — Rectangle packing report

### Brief

Model rectangular packages and determine whether one package can fit completely inside another without rotation.

### Required behavior

- Define a `Rectangle` with unsigned width and height.
- Add `area`, `perimeter`, `can_hold`, and `square` methods or associated functions.
- Reject zero dimensions through a constructor that returns `Result`.
- `can_hold` must require both dimensions to be strictly larger.

### Expected interface and `main.rs` flow

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Result<Self, String>;
    fn square(size: u32) -> Result<Self, String>;
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
    fn can_hold(&self, other: &Rectangle) -> bool;
}
```

`main` builds the outer and inner rectangles through `Rectangle::new`, propagates construction errors, and uses the methods to print the report. It must not construct unchecked rectangles with struct literals.

### Example

```text
outer: 30 x 20
inner: 10 x 15
outer area: 600
outer can hold inner: true
```

### Verification checklist

- Verify area of `30 × 20` is `600`.
- Verify perimeter of `30 × 20` is `100`.
- Verify `30 × 20` holds `10 × 15`.
- Verify equal dimensions do not count as strictly contained.
- Verify a zero dimension is rejected.


- Verify a report created from two valid rectangles contains all calculated fields.

### Done when

Construction protects the type’s invariant and instance behavior is expressed through methods.

<!-- /challenge -->
