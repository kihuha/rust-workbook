# Chapter 5 — Structs, enums, and pattern matching

## Challenge 013 — Bank account record

**Mini brief:** Build the data model for a simple bank account and print a customer-friendly summary.

**Practice:** Struct fields, ownership inside structs, and implementation methods.

**Requirements:** Define `Account` with an owner name, account number, and balance in cents. Add `new` and `summary` methods. Create two accounts and show that each stores its own data.

**Done when:** Construction and display use methods, and money is represented without floating-point rounding.

**Stretch:** Add a method that changes the owner's display name while preserving the account number.

## Challenge 014 — Transaction outcomes

**Mini brief:** A transfer may succeed, fail for insufficient funds, or fail because the destination account is missing.

**Practice:** Enums, associated data, and `match` exhaustiveness.

**Requirements:** Define a `TransferResult` enum with useful details for each outcome. Write a function that produces a result and a `match` that prints a clear message for every variant.

**Done when:** Each outcome carries enough data to explain what happened, and no wildcard arm hides a case.

**Stretch:** Include the amount and account identifier in the success variant.

## Challenge 015 — Shipping quote

**Mini brief:** A parcel service offers pickup, standard delivery, or express delivery, each with different fee rules.

**Practice:** Enums as domain types, pattern matching, and methods on enums.

**Requirements:** Define a `Delivery` enum and implement `fee_cents` based on weight and delivery mode. Reject a zero or unreasonable weight using a result type or a validation function.

**Done when:** All delivery modes are handled explicitly and sample fees are easy to check by hand.

**Stretch:** Add a textual description method without duplicating the fee logic.
