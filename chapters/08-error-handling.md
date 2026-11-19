# Chapter 8 — Error handling

## Challenge 022 — Validated deposit

**Challenge:** A deposit can fail because the text is not a number or because the parsed amount violates a business rule. Preserve that distinction.

**Your task:** Implement `parse_deposit(input: &str) -> Result<u64, DepositError>`. Parse whole cents and reject zero and negative values.

**Example results:**

```text
"2500" -> Ok(2500)
"abc"  -> Err(InvalidNumber)
"0"    -> Err(NonPositive)
"-50"  -> Err(NonPositive)
```

Leading and trailing whitespace may be accepted after trimming.

**Requirements:** Use a custom error enum with distinct variants. Keep terminal input outside the parser so the function can be tested directly.

**Done when:** Every example produces the documented category and valid data is returned as cents.

**Stretch:** Implement `Display` and `Error` for `DepositError`.

## Challenge 023 — CSV expense loader

**Challenge:** A loader must distinguish file failures from malformed rows and report exactly where bad data occurred.

**Your task:** Read a file whose non-empty lines use `description,amount_cents`. Return `Vec<Expense>` or an error containing useful context.

**Example file:**

```text
Bus,150
Lunch,800
Books,not-a-number
```

**Expected error:**

```text
line 3: invalid amount "not-a-number"
```

**Requirements:** Reject missing descriptions, missing amounts, extra fields, and invalid numbers. Report line numbers starting at 1. Represent a missing file separately. Do not `unwrap` expected failures.

**Done when:** A valid file loads all rows and every malformed row identifies its location.

**Stretch:** Support quoted descriptions containing commas by documenting a rule or adopting a CSV crate.

## Challenge 024 — Safe calculator

**Challenge:** Parse and evaluate an arithmetic request without panicking on bad input, zero division, or overflow.

**Your task:** Implement `calculate(left: &str, operator: &str, right: &str) -> Result<i64, CalcError>` for `+`, `-`, `*`, and `/`.

**Example results:**

```text
calculate("12", "+", "8") -> Ok(20)
calculate("9", "/", "0")  -> Err(DivisionByZero)
calculate("x", "+", "2")  -> Err(InvalidLeftOperand)
calculate("4", "%", "2")  -> Err(UnknownOperator)
```

**Requirements:** Distinguish invalid left and right operands. Use checked arithmetic, including checked division for `i64::MIN / -1`.

**Done when:** Each operator works and every failure path returns a specific error.

**Stretch:** Add remainder and document the result for negative operands.
