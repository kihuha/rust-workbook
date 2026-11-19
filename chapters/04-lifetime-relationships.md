# Chapter 4 — Lifetime relationships

These challenges focus on whether references remain valid. Add explicit lifetime syntax only where the compiler requires a relationship to be named.

## Challenge 010 — Choose the longer label

**Challenge:** A helper returns one of two borrowed strings. Rust needs to know how the returned reference is related to both inputs.

**Your task:** Implement `longer_label` so it returns the input with the greater character count. If the counts are equal, return the first input.

**Example inputs and outputs:**

```text
("Rust", "Ownership") -> "Ownership"
("book", "page")      -> "book"
```

**Requirements:** Write down what must stay alive while the result is used. Add the lifetime annotations Rust requires. Do not create a local `String` and return a reference to it.

**Done when:** Both cases work and you can explain why the result may not outlive either input lifetime described by the signature.

**Stretch:** Return an owned `String` instead and compare its allocation cost with the caller's simpler lifetime obligations.

## Challenge 011 — Catalog search result

**Challenge:** Search a collection and return the original matching title without allocating a duplicate.

**Your task:** Store titles in `Vec<String>` and implement a prefix search that returns `Option<&str>` borrowed from the catalog.

**Example catalog:** `['The Rust Book', 'Rust in Action', 'Programming Rust']`.

**Example results:**

```text
prefix "Rust"   -> Some("Rust in Action")
prefix "Python" -> None
```

**Requirements:** Return the first match. Keep the catalog alive while using the result. Add a commented invalid scope arrangement in which the result would outlive the catalog.

**Done when:** Found and missing cases work, and the successful result points into the existing catalog data.

**Stretch:** Match prefixes case-insensitively while still returning the title with its original spelling.

## Challenge 012 — Borrow or own?

**Challenge:** A parsed command may borrow a token from the input, but an empty input needs a generated default. One result type must safely represent both sources.

**Your task:** Implement `command(input: &str)` using either an owned `String`, `Cow<'_, str>`, or a small enum with `Borrowed(&str)` and `Owned(String)` variants.

**Example inputs and outputs:**

```text
"list files" -> "list"
"   "        -> "help"
```

**Requirements:** The non-empty result should use the first whitespace-separated token. The default must not return a reference to a temporary local `String`. Explain the ownership cost of your chosen result type.

**Done when:** Both cases work without a dangling reference and the caller can print the result safely.

**Stretch:** Implement the `Cow<'a, str>` version so non-empty input is borrowed and the generated default is owned.
