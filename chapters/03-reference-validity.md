# Chapter 3 — Reference validity, scopes, and non-lexical lifetimes

## Challenge 007 — Borrow timeline notebook

**Challenge:** A score update succeeds or fails depending on the last use of a reference, not just the braces around it. Make that borrow timeline visible.

**Your task:** Start with `score = 70`. Borrow it as `current`, print through the reference, add `5` to the owner, and print the owner.

**Requirements:** Then create a second version that uses `current` again after the update. Predict whether each version compiles before checking. Draw a timeline marking creation, last use, mutation, and scope end.

**Expected output for the valid version:**

```text
Before: 70
After: 75
```

The invalid version should demonstrate that mutation cannot overlap a later use of the immutable borrow.

**Done when:** You can explain why non-lexical lifetimes let a borrow end at its last use.

**Stretch:** Put the borrow in an inner block and compare the explicit block boundary with the last-use boundary inferred by Rust.

## Challenge 008 — Safe first word

**Challenge:** Return a view into a log line without copying it, while ensuring that view never outlives its source.

**Your task:** Implement `first_word(line: &str) -> &str`. Return the text before the first ASCII space, or the entire input if no space exists.

**Example inputs and outputs:**

```text
"error disk full" -> "error"
"ready"           -> "ready"
""                -> ""
" two"            -> ""
```

**Requirements:** Test all four cases. Keep the source `String` alive while printing the returned slice. Add a commented failing example that tries to keep the slice after its source is dropped.

**Done when:** No new `String` is allocated for the result, and you can identify the byte range in the source that the slice refers to.

**Stretch:** Treat whitespace and punctuation as delimiters without slicing in the middle of a UTF-8 character.

## Challenge 009 — Edit after inspection

**Challenge:** A note can be mutated after a preview borrow's last use, but not while that preview is still needed.

**Your task:** Create `note = "Draft report"`. Borrow a preview, print it, append `" [saved]"` to the owner, and print the final note.

**Expected output:**

```text
Preview: Draft report
Saved: Draft report [saved]
```

**Requirements:** Make a second version that tries to print the preview after appending. Record why it fails, then restore compiling code. Mark the borrow's last use in both timelines.

**Done when:** The valid version compiles and your explanation connects the mutation rule to the preview's actual last use.

**Stretch:** Move preview creation into a helper that returns an owned summary. Explain why the returned summary no longer borrows the note.
