# Chapter 5 — Structs, enums, and pattern matching

## Challenge 013 — Bank account record

**Challenge:** Model a bank account so related data and behavior live together, and money is represented without floating-point errors.

**Your task:** Define `Account` with `owner: String`, `account_number: String`, and `balance_cents: i64`. Add `new` and `summary` methods and create two independent accounts.

**Example account:** Owner `Amina Njeri`, number `AC-1001`, balance `125050` cents.

**Expected summary:**

```text
AC-1001 | Amina Njeri | balance: 1250.50
```

**Requirements:** `new` must initialize every field. `summary` must borrow `self`, not consume the account. Demonstrate that both accounts keep their own owner and balance.

**Done when:** The methods construct and display both accounts, and the original accounts remain usable after calling `summary`.

**Stretch:** Add `rename_owner(&mut self, new_name: String)` and prove that the account number remains unchanged.

## Challenge 014 — Transaction outcomes

**Challenge:** A transfer can finish in several distinct ways. Represent every outcome with enough data to explain it, then handle all cases explicitly.

**Your task:** Define `TransferResult` variants for success, insufficient funds, and a missing destination. Write `transfer` and a `match` that prints a message for each variant.

**Example inputs and outputs:**

```text
balance 5000, amount 1200, destination exists
-> "Transferred 1200 cents; remaining balance: 3800 cents"

balance 500, amount 1200, destination exists
-> "Insufficient funds: needed 1200, available 500"

destination "AC-404" missing
-> "Destination AC-404 was not found"
```

**Requirements:** Carry the relevant values inside the enum variants. Do not use a wildcard match arm.

**Done when:** All three outcomes can be produced and the compiler verifies that every variant is handled.

**Stretch:** Add a rejected-zero-amount outcome without changing the meaning of existing variants.

## Challenge 015 — Shipping quote

**Challenge:** Delivery modes use different pricing rules. Encode the modes as a domain type and calculate fees without scattered conditionals.

**Your task:** Define `Delivery::{Pickup, Standard, Express}` and implement `fee_cents(&self, weight_grams: u32) -> Result<u32, QuoteError>`.

**Use these rules:** Pickup is free. Standard costs `500 + 2` cents per started `100 g`. Express costs `1200 + 5` cents per started `100 g`. Valid weights are `1..=50_000 g`.

**Example results:**

```text
Pickup, 850 g   -> Ok(0)
Standard, 850 g -> Ok(518)
Express, 850 g  -> Ok(1245)
Standard, 0 g   -> Err(InvalidWeight)
```

**Requirements:** Match every delivery mode explicitly. Round partial 100-gram units up, and return an error for weights outside the valid range.

**Done when:** Every mode is matched explicitly, rounding uses started 100-gram units, and invalid weights return an error instead of panicking.

**Stretch:** Add a `description` method without duplicating the fee calculation.
