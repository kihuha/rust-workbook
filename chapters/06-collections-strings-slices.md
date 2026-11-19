# Chapter 6 — Collections, strings, and slices

## Challenge 016 — Daily expense summary

**Challenge:** Summarize a collection without consuming it or assuming it contains an item.

**Your task:** Store expenses as `Vec<(String, u32)>`, where the number is cents. Compute the total, the largest item, and the number of expenses strictly above a threshold.

**Example input:** Groceries `4500`, bus `150`, lunch `800`; threshold `500`.

**Expected result:**

```text
Total: 5450 cents
Largest: Groceries (4500 cents)
Above 500 cents: 2
```

**Requirements:** Summary helpers must borrow the slice. For an empty list, return total `0`, count `0`, and no largest item. Define how ties for largest are resolved.

**Done when:** Empty, one-item, and multi-item inputs work without unchecked indexing.

**Stretch:** Produce a view sorted from highest to lowest amount while preserving the original vector order.

## Challenge 017 — Word statistics

**Challenge:** Compute text statistics safely without byte indexing or division by zero.

**Your task:** Write a function that accepts `&str` and returns the word count, longest word, and average word length. Split words with `split_whitespace`.

**Example input:** `"Rust makes systems safer"`.

**Expected result:**

```text
Words: 4
Longest: systems
Average length: 5.25
```

Lengths in the base challenge are UTF-8 byte lengths. For equal longest words, keep the first.

**Requirements:** Multiple spaces must not create empty words. Empty input must return an explicit no-statistics result rather than dividing by zero.

**Done when:** The example, repeated whitespace, and empty input all have defined results.

**Stretch:** Count Unicode scalar values instead of bytes and explain how grapheme clusters can still differ.

## Challenge 018 — Frequency table

**Challenge:** Count normalized words, then sort a map's entries into deterministic report order.

**Your task:** Lowercase each whitespace-separated word, count it in a `HashMap<String, usize>`, and print entries by descending count. Break ties alphabetically.

**Example input:** `"Rust is fast and rust is safe"`.

**Expected output:**

```text
is: 2
rust: 2
and: 1
fast: 1
safe: 1
```

**Requirements:** Use the entry API for counting. Do not rely on `HashMap` iteration order. Sorting a separate vector of borrowed entries is acceptable.

**Done when:** Repeated words are counted case-insensitively and equal counts always print in the same order.

**Stretch:** Strip surrounding punctuation while preserving apostrophes inside words such as `don't`.
