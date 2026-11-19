# 02 — Cybersecurity: ArgonForge Argon2id password hasher

## Project warning

This is an advanced cryptographic-engineering project. Build the algorithm to
understand memory-hard password hashing, careful binary formats, integer
arithmetic, parallel memory filling, and security boundaries.

Do **not** deploy the implementation to protect real credentials. A result that
matches test vectors is not automatically resistant to side channels, compiler
optimizations, denial of service, memory-disclosure bugs, or cryptanalytic
mistakes. Production applications should use a mature, audited Argon2 library.

## Scenario

ArgonForge is a fictional identity-platform team replacing a fast, unsalted
password digest. It needs an interoperable password-hashing component based on
[RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html), which specifies Argon2
version 1.3 (`v=19`).

Build a Rust library and a small diagnostic CLI that implement **Argon2id** from
the specification. The library must produce raw tags, encode and verify PHC
password-hash strings, enforce application resource limits, and explain when a
stored hash needs to be upgraded.

Argon2id is the required variant because RFC 9106 requires implementations to
support it and recommends it when side-channel threats are possible or the
caller is unsure which variant to choose. Argon2d and Argon2i are optional
extensions after Argon2id is correct.

## Learning outcomes

By completing the project, you should be able to:

- explain why password hashing must be salted, deliberately slow, and
  memory-hard;
- translate a cryptographic specification into explicit byte-level operations;
- distinguish the password, salt, optional secret, and associated-data inputs;
- implement checked parameter validation before allocating attacker-controlled
  memory;
- model Argon2's lanes, four slices per lane, segments, passes, and reference
  blocks;
- implement the BLAKE2b-based `H'` function and Argon2's `G` compression
  function without confusing them with ordinary BLAKE2b hashing;
- preserve deterministic output while introducing lane-level parallelism;
- store and parse self-describing PHC strings safely; and
- establish confidence with official vectors, differential tests, fuzzing, and
  benchmarks rather than with a few example passwords.

## Brown Rust Book prerequisites

Study [ownership and borrowing](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html),
[structs](https://rust-book.cs.brown.edu/ch05-00-structs.html),
[enums and matching](https://rust-book.cs.brown.edu/ch06-00-enums.html),
[modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html),
[collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html),
[error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html),
[testing](https://rust-book.cs.brown.edu/ch11-00-testing.html),
[iterators](https://rust-book.cs.brown.edu/ch13-00-functional-features.html), and
[concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html).

You should already be comfortable with hexadecimal notation, little-endian
encoding, XOR, wrapping integer arithmetic, bit rotation, and reading pseudocode.

## Source hierarchy

Use sources according to the question being answered. A deployment guide cannot
override the algorithm specification, and a passing result from one library is
not a substitute for the RFC vectors.

| Priority | Source | Use it for |
| --- | --- | --- |
| 1 | [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html) and its [verified errata](https://www.rfc-editor.org/errata/eid7721) | Normative Argon2 v1.3 inputs, operations, indexing, parameter guidance, security analysis, and vectors |
| 2 | [RFC 7693](https://www.rfc-editor.org/rfc/rfc7693.html) | The underlying BLAKE2b definition and test vectors |
| 3 | [Argon2 paper](https://eprint.iacr.org/2015/430) and [full specification](https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf) | Design rationale, attacks, indexing derivation, and details that explain the implementer-oriented RFC |
| 4 | [Pinned PHC Argon2 reference revision](https://github.com/P-H-C/phc-winner-argon2/tree/f57e61e19229e23c4445b85494dbf7c07de721cb) and its [Argon2id KAT corpus](https://github.com/P-H-C/phc-winner-argon2/blob/f57e61e19229e23c4445b85494dbf7c07de721cb/kats/argon2id) | Reproducible intermediate-value comparison, interoperability, and behavior clarification |
| 5 | [C2SP PHC Strings specification](https://c2sp.org/phc-strings) | Grammar and encoding of stored password-hash strings |
| 6 | [RustCrypto Argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2) | An independent Rust differential oracle and an example of a reviewed crate boundary |

RFC 9106 defines this project's required algorithm behavior. Use the reference C
and Rust implementations to clarify and compare results, not as code to
transliterate line by line. When sources appear to disagree, record the exact
inputs, versions, source sections, and resolution in `docs/spec-notes.md`; do not
silently choose the output that makes a test pass.

The annotated source map in
[`research/argon2-implementation-guidance.md`](../../../research/argon2-implementation-guidance.md)
explains what each reference contributes and where its authority ends.

## Additional implementation and deployment guidance

These sources complement the normative algorithm documents:

| Concern | Guidance |
| --- | --- |
| Rust API and interoperability | [`argon2` crate documentation](https://docs.rs/argon2/latest/argon2/) and [`password-hash` crate documentation](https://docs.rs/password-hash/latest/password_hash/) |
| BLAKE2b primitive | [`blake2` crate documentation](https://docs.rs/blake2/latest/blake2/) |
| Constant-time operations | [`subtle` crate documentation](https://docs.rs/subtle/latest/subtle/) |
| Best-effort secret erasure | [`zeroize` crate documentation](https://docs.rs/zeroize/latest/zeroize/) |
| Password-verifier requirements | [NIST SP 800-63B](https://pages.nist.gov/800-63-4/sp800-63b.html) |
| Argon2id policy guidance | [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html) |
| Operational profiles and rehashing | [libsodium password-hashing documentation](https://doc.libsodium.org/password_hashing/default_phf) |
| Emerging password-storage guidance | [IETF password-storage draft status](https://datatracker.ietf.org/doc/draft-ietf-kitten-password-storage/) |

Treat library documentation as versioned evidence. Pin the dependency versions
used for differential tests. Every imported or generated fixture must record
its source URL, source revision or crate version, variant, algorithm version,
parameters, and checksum. Treat NIST, OWASP, libsodium, and the active IETF
Internet-Draft as deployment guidance: their operational recommendations can
evolve independently of Argon2 v1.3's byte-for-byte definition. An
Internet-Draft is not a finished standard.

## Scope and non-goals

### Required

- Argon2 version 1.3 (`0x13`, encoded as `v=19`).
- The Argon2id variant (`y = 2`).
- Single-threaded correctness for every valid degree of parallelism supported by
  your application policy.
- Parallel lane filling without changing the output.
- Raw-tag generation.
- PHC string encoding, parsing, verification, and rehash decisions.
- A command-line tool for controlled experiments and interoperability checks.
- Secure-by-default application limits around the much wider RFC parameter
  ranges.

### Not required

- Version 1.0 (`v=16`) compatibility.
- SIMD, assembly, GPU, or platform-specific acceleration.
- A replacement for an audited password-hashing crate.
- A login server, database, or network API.
- Perfect guarantees that sensitive bytes are erased from every compiler-created
  copy.

### Optional

- Argon2i and Argon2d.
- Optional secret input (often called a pepper) and associated data.
- A no-standard-library build.
- SIMD selected through safe runtime feature detection.

## Deliverables

Create one Cargo workspace with these components:

```text
argonforge/
├── Cargo.toml
├── crates/
│   ├── argonforge-core/      # Algorithm, typed parameters, raw tags
│   ├── argonforge-password/  # Salt generation, PHC strings, verify/rehash
│   └── argonforge-cli/       # Diagnostic CLI
├── tests/
│   └── vectors/              # Versioned RFC and differential-test fixtures
├── fuzz/                     # Parser and parameter-validation targets
├── benches/                  # Cost and scaling measurements
├── docs/
│   ├── algorithm.md
│   ├── security.md
│   └── spec-notes.md
└── README.md
```

Keep the algorithmic core independent of CLI parsing, random-number generation,
PHC text encoding, and operating-system concerns.

## Public library contract

Design an API with the following responsibilities. Exact names may differ, but
the boundaries must remain clear.

```rust
pub struct Params {
    pub memory_kib: u32,
    pub passes: u32,
    pub lanes: u32,
    pub tag_len: usize,
}

pub struct ResourceLimits {
    pub max_memory_kib: u32,
    pub max_passes: u32,
    pub max_lanes: u32,
    pub max_tag_len: usize,
}

pub fn derive_argon2id(
    password: &[u8],
    salt: &[u8],
    params: &Params,
    limits: &ResourceLimits,
) -> Result<Vec<u8>, Error>;

pub fn hash_password(
    password: &[u8],
    policy: &PasswordPolicy,
    rng: &mut impl CryptoRng,
) -> Result<String, Error>;

pub fn verify_password(
    password: &[u8],
    encoded: &str,
    limits: &ResourceLimits,
) -> Result<Verification, Error>;
```

`Verification` should distinguish at least:

- password matched and the parameters are current;
- password matched but the hash should be replaced under the current policy;
- password did not match; and
- the stored value is malformed or violates resource limits.

Do not accept a plain `Params` value without validation. Prefer a validated type
whose constructor proves the invariants used by the filling algorithm.

## Inputs and invariants

RFC 9106 defines these inputs:

| Name | Meaning | Project requirement |
| --- | --- | --- |
| `P` | Password/message bytes | Treat as arbitrary bytes, not necessarily UTF-8 |
| `S` | Salt/nonce bytes | Generate a unique random 16-byte salt for new password hashes |
| `p` | Parallelism/lanes | Validate before multiplication or allocation |
| `T` | Output tag length in bytes | Support at least 16–64 bytes; default to 32 |
| `m` | Requested memory in KiB | Require `m >= 8p`; apply an application maximum |
| `t` | Number of passes | Require at least one; apply an application maximum |
| `v` | Version | Require `0x13` |
| `K` | Optional secret | Optional extension; never encode it in the PHC string |
| `X` | Optional associated data | Optional extension; document how the verifier obtains it |
| `y` | Variant | Require `2` for Argon2id |

The RFC permits values far too large to accept from untrusted stored strings.
Validation therefore has two layers:

1. **Algorithm validity** enforces the RFC relationships and integer ranges.
2. **Application policy** caps memory, passes, lanes, salt length, encoded-string
   length, and tag length before expensive work begins.

Use checked conversions and checked multiplication when computing block counts,
byte counts, lane lengths, segment lengths, and allocation sizes. Return a typed
error instead of panicking or silently truncating.

The effective memory-block count is:

```text
m' = 4 * p * floor(m / (4 * p))
```

Each block is exactly 1024 bytes, represented as 128 little-endian `u64` words.
The lane length is `q = m' / p`; each of the four segments in a lane has length
`q / 4`.

## Algorithm requirements

### 1. Initial hash `H₀`

Construct the initial 64-byte digest from the fields in the exact RFC order:

```text
LE32(p) || LE32(T) || LE32(m) || LE32(t) ||
LE32(v) || LE32(y) ||
LE32(len(P)) || P ||
LE32(len(S)) || S ||
LE32(len(K)) || K ||
LE32(len(X)) || X
```

Hash that byte sequence with BLAKE2b configured for a 64-byte output. Length
fields remain present even when an optional input is empty. Write focused tests
for field order, little-endian encoding, empty inputs, and non-UTF-8 bytes.

You may use a well-tested BLAKE2b primitive. Do not use an existing Argon2
implementation inside `argonforge-core`.

### 2. Variable-length hash `H'`

Implement RFC 9106 Section 3.3 independently from the memory-filling code.

- Prefix the input with `LE32(T)`.
- For `T <= 64`, request exactly `T` bytes from BLAKE2b.
- For `T > 64`, emit 32-byte prefixes from chained 64-byte digests and finish
  with the precisely sized final digest.

Test boundary lengths `4`, `32`, `63`, `64`, `65`, `96`, and `1024`. Off-by-one
errors in `H'` will corrupt both starting blocks and final tags.

### 3. Block and compression function `G`

Define a block as 128 `u64` words. Decode and encode each word explicitly as
little-endian; do not rely on host endianness or unsafe pointer casts.

For two blocks `X` and `Y`:

1. Compute `R = X XOR Y`.
2. Copy `R` to `Z`.
3. Apply the Argon2 permutation row-wise to `Z`.
4. Apply it column-wise to `Z`.
5. Return `Z XOR R`.

The permutation uses the BLAKE2 round layout but replaces ordinary addition
with the BlaMka operation:

```text
a + b + 2 * low32(a) * low32(b) mod 2^64
```

Use deliberate `wrapping_add` and `wrapping_mul` operations. Implement and test
the four-word `GB` mixing function, the 16-word permutation, and the full-block
row/column schedule separately. Include values near `u64::MAX` so debug and
release builds agree.

### 4. Initialize lanes

Allocate `m'` blocks and view them as `p` lanes. For every lane `i`, compute:

```text
B[i][0] = H'^1024(H₀ || LE32(0) || LE32(i))
B[i][1] = H'^1024(H₀ || LE32(1) || LE32(i))
```

Keep the mapping from `(lane, column)` to the backing allocation explicit and
tested. Incorrect flattening can produce plausible but non-standard hashes.

### 5. Compute addresses and reference blocks

Split every lane into four segments. Process the matrix in this order:

```text
for each pass
    for each slice 0..4
        process that slice in every lane
```

Segments in the same slice may run in parallel, but all lanes must finish the
slice before any lane starts the next one.

Argon2id uses data-independent Argon2i-style addresses during pass 0, slices 0
and 1. It uses data-dependent Argon2d-style addresses for all remaining
segments. Implement the two address sources behind one small interface so the
transition is visible and testable.

Map `J₁` and `J₂` to a reference lane and the non-uniform reference-block
position exactly as described in RFC 9106 Section 3.4. Treat these as named
intermediate values. Test:

- the first and later passes;
- all four slices;
- same-lane and cross-lane references;
- the first block of a segment;
- the special first-pass, first-slice restriction to the current lane; and
- the exclusion of blocks that are not yet legal to reference.

Do not simplify the reference area until the unsimplified version matches the
official vector. Indexing is the most error-prone part of this project.

### 6. Fill memory

On the first pass, each new block is the compression result of the preceding
block and selected reference block. On later passes, XOR the newly computed
value with the block's previous contents as specified by RFC 9106.

First produce a correct deterministic implementation that processes lanes in a
fixed order. Then introduce parallel workers at the segment level with a barrier
between slices. The single-threaded and parallel engines must produce identical
tags for the same logical `p` value; `p` describes the Argon2 instance, not just
the number of worker threads available.

Avoid data races without copying the entire memory matrix. If safe disjoint
borrows cannot express the schedule directly, encapsulate the synchronization
design in one small module and document its invariants before considering any
`unsafe` code. A fully safe implementation is preferred.

### 7. Finalize the tag

XOR the last block of every lane into one 1024-byte block `C`, then compute
`H'^T(C)`. Return exactly `T` bytes.

Internal memory should be cleared when it is no longer needed. Document the
limits of zeroization in optimized Rust and ensure debug output never exposes
passwords, secret inputs, memory blocks, or raw tags unless an explicitly
selected diagnostic mode is operating on public test fixtures.

## Password storage layer

### PHC string format

For normal password storage, emit a self-describing string shaped like:

```text
$argon2id$v=19$m=65536,t=3,p=4$<salt-base64>$<tag-base64>
```

Follow the [C2SP PHC Strings specification](https://c2sp.org/phc-strings): use
the prescribed unpadded Base64 for the salt and tag, canonical decimal
parameters, and the Argon2 field order. Do not invent a private database format.

The parser must reject:

- algorithms other than `argon2id` in the required implementation;
- missing, duplicated, unknown, or reordered fields if your grammar forbids
  them;
- versions other than `19`;
- non-decimal, signed, zero, overflowing, or non-canonical parameter values;
- invalid or padded Base64 when canonical unpadded encoding is required;
- Base64 with whitespace, an invalid length, or non-zero unused trailing bits;
- salts or tags outside policy limits; and
- encoded strings above a small documented length limit.

Parse and validate the entire string before allocating the Argon2 memory region.

### Hashing a new password

- Obtain a fresh 16-byte salt from an operating-system-backed cryptographic RNG.
- Never reuse a fixed salt outside fixtures.
- Apply a documented `PasswordPolicy` rather than accepting caller-controlled
  costs for the normal high-level operation.
- Return only the encoded PHC string; callers do not need the raw tag.
- Never log the password, salt-generation state, or raw working memory.

RFC 9106 offers two uniformly safe profiles: Argon2id with 2 GiB, one pass, and
four lanes; or the lower-memory profile with 64 MiB, three passes, and four
lanes. Treat these as specification guidance, not timeless deployment
constants. Make the policy configurable and benchmark it on the actual target
hardware under expected concurrency and memory pressure.

### Verification

1. Parse the stored string under strict resource limits.
2. Derive a candidate tag using the stored parameters and salt.
3. Compare candidate and stored tags in constant time.
4. Return one generic mismatch result; do not reveal a useful partial match.
5. If verification succeeds, report whether the stored algorithm, version, or
   costs are weaker than the current policy so the caller can rehash after
   authentication.

Do not lower application limits merely because a stored database row requests a
larger value. A malicious PHC string must not cause unbounded CPU use, memory
allocation, thread creation, or integer overflow.

## CLI interface

Provide a diagnostic binary named `argonforge`:

| Command | Purpose |
| --- | --- |
| `argonforge hash --memory-kib <m> --passes <t> --lanes <p>` | Read a password without echo and print a PHC string |
| `argonforge verify --encoded <phc>` | Read a password without echo and report match, mismatch, or invalid hash |
| `argonforge inspect --encoded <phc>` | Parse and display non-secret metadata without hashing |
| `argonforge vector <file>` | Run a public test-vector fixture and show named checkpoints |
| `argonforge bench --profile <name>` | Measure a policy on this machine without accepting a real password |

Passwords must not be accepted as ordinary command-line arguments because they
can appear in shell history and process listings. Support an interactive hidden
prompt and an explicit standard-input mode for tests. Clearly label any raw-tag
output as diagnostic-only.

Example:

```text
$ argonforge inspect --encoded '$argon2id$v=19$m=65536,t=3,p=4$...$...'
algorithm: argon2id
version: 19
memory: 65536 KiB
passes: 3
lanes: 4
tag length: 32 bytes
within local verification limits: yes
```

## Error model

Use structured errors and keep these categories distinguishable to library
callers:

- invalid algorithm parameters;
- local resource-limit violation;
- malformed PHC encoding;
- unsupported algorithm or version;
- cryptographic RNG failure;
- allocation failure where it can be handled;
- worker failure; and
- password mismatch as a verification outcome, not an internal exception.

CLI messages may explain malformed hashes, but an authentication service built
on the library should normally map mismatch and invalid stored credentials to a
single external response.

## Implementation milestones

### Milestone 1 — Specification map and types

Write `docs/algorithm.md` with a glossary and a diagram of passes, lanes,
slices, and segments. Define validated parameters, checked size calculations,
blocks, and typed errors. No large allocation should occur yet.

### Milestone 2 — Hash primitives

Implement little-endian helpers, `H'`, BlaMka, `GB`, the 16-word permutation,
and block compression. Add boundary and overflow tests before building the
matrix.

### Milestone 3 — Initial digest and lane starts

Implement `H₀`, effective-memory rounding, matrix allocation, and the first two
blocks of every lane. Compare intermediate values with the RFC vector.

### Milestone 4 — Indexing engine

Implement data-independent address-block generation, data-dependent address
extraction, reference-lane selection, and reference-position mapping. Create
small synthetic tests for every addressing branch.

### Milestone 5 — Sequential Argon2id

Fill every pass and slice in a deterministic lane order, finalize the last
column, and match the complete RFC 9106 Argon2id vector before adding threads or
PHC strings.

### Milestone 6 — Interoperability

Add PHC encoding and strict parsing. Differentially test generated raw tags and
encoded values against the reference implementation or an independent mature
Rust implementation over many small parameter combinations.

### Milestone 7 — Parallel execution

Run same-slice lane segments concurrently with a barrier at each slice. Prove
equivalence to the sequential engine with randomized tests and ThreadSanitizer
or another suitable race-detection strategy where available.

### Milestone 8 — Password workflow hardening

Add secure salt generation, hidden CLI input, constant-time verification,
rehash decisions, zeroization, policy limits, fuzz targets, and security
documentation.

### Milestone 9 — Performance characterization

Benchmark memory, pass, and lane scaling in release mode. Measure latency,
throughput, peak resident memory, and behavior under concurrent verification.
Performance work may begin only after all correctness tests pass.

## Required tests

### Primitive tests

- `T01` — `LE32`/`LE64` encoding is independent of host endianness.
- `T02` — `H'` matches trusted fixtures below, at, and above 64 bytes.
- `T03` — BlaMka and `GB` use wrapping arithmetic and exact rotation counts.
- `T04` — Row-wise and column-wise permutations touch every intended word once.
- `T05` — Full-block `G` matches an independently generated fixture.

### RFC conformance tests

- `T06` — The RFC 9106 Argon2id `H₀` checkpoint matches exactly.
- `T07` — The documented first and last words of pass 0 match.
- `T08` — The documented first and last words of passes 1 and 2 match.
- `T09` — The final 32-byte RFC Argon2id tag is
  `0d640df58d78766c08c037a34a8b53c9d01ef0452d75b65eb52520e96b01e659`.
- `T10` — Requested memory that is not divisible by `4p` is rounded exactly as
  the RFC specifies while `H₀` still commits to the requested `m` value.

### Indexing and scheduling tests

- `T11` — Argon2id uses independent addresses only in pass 0, slices 0 and 1.
- `T12` — The first two lane blocks are never overwritten during their initial
  segment.
- `T13` — Every selected reference belongs to the legal reference area.
- `T14` — Later passes XOR with the old destination block.
- `T15` — Sequential and parallel engines agree for multiple `m`, `t`, `p`,
  password, salt, and tag-length combinations.

### Encoding and verification tests

- `T16` — PHC strings round-trip to one canonical representation.
- `T17` — Changing the password, salt, costs, version, or tag changes or
  invalidates verification as appropriate.
- `T18` — Tag comparison uses a reviewed constant-time primitive.
- `T19` — Successful verification requests rehash under a stronger current
  policy and does not request it under an equivalent policy.
- `T20` — Empty, non-UTF-8, and long passwords are handled as documented.

### Abuse and robustness tests

- `T21` — Oversized memory, pass, lane, tag, salt, and encoded-string values are
  rejected before the expensive allocation.
- `T22` — Arithmetic boundary cases return errors without panic in debug or
  release builds.
- `T23` — Fuzzing the PHC parser and parameter validator finds no panic or
  unbounded allocation.
- `T24` — Logs, errors, panic messages, and default debug formatting do not
  contain password or secret bytes.
- `T25` — Malformed Base64, non-zero unused trailing bits, duplicate parameters,
  non-canonical decimals, unknown fields, and unsupported versions are rejected
  deterministically.

### Differential and quality tests

- `T26` — At least 100 generated low-cost cases match both a pinned reference C
  build and a pinned independent Rust Argon2id implementation byte for byte;
  fixture provenance and checksums are retained.
- `T27` — Results match on little-endian and big-endian targets, using emulation
  in CI if necessary.
- `T28` — `cargo fmt`, strict `clippy`, tests, documentation tests, and dependency
  auditing pass in CI.
- `T29` — Miri covers safe primitive and parser tests; any `unsafe` block has a
  written invariant and focused tests.

Keep differential-test costs deliberately small in ordinary CI. Run the real
deployment policy in a separate benchmark or scheduled job so tests remain
usable.

## Benchmark report

Produce a report containing:

- compiler version, optimization settings, CPU, logical cores, and RAM;
- median and tail latency for each policy;
- observed peak memory versus requested `m`;
- scaling when doubling memory and passes;
- scaling from one lane to several lanes;
- sequential versus parallel engine results; and
- concurrent verification behavior at realistic service load.

Do not choose costs from a single developer-laptop measurement. Explain how a
service would budget total memory as approximately `concurrent_hashes × memory
per hash`, reserve headroom, limit authentication concurrency, and prevent an
attacker from exhausting the host through repeated login attempts.

## Security review checklist

Before declaring the project complete, answer these questions in
`docs/security.md`:

- Can an untrusted PHC string trigger excessive allocation, CPU work, or thread
  creation before limits are checked?
- Are all byte orders, lengths, and wrapping operations explicit?
- Is every password hash given a unique unpredictable salt?
- Are passwords excluded from arguments, logs, debug output, traces, and error
  messages?
- Is verification constant-time with respect to tag contents?
- Does parallel scheduling preserve the slice barrier and legal reference area?
- What sensitive buffers are wiped, and what copies may remain outside the
  implementation's control?
- How are policies upgraded without locking out users?
- What happens on allocation failure, worker panic, RNG failure, or malformed
  stored data?
- Which modules contain `unsafe`, and what invariants make each block sound?
- How is dependency provenance checked and how are security updates handled?
- When will maintainers recheck RFC errata, operational recommendations,
  dependency versions, and the status of active password-storage standards?
- Why is this implementation still educational rather than production-approved?

## Acceptance criteria

The project is complete when:

1. Argon2id v1.3 raw output matches every required RFC checkpoint and final
   vector.
2. Differential tests agree with an independent implementation across varied
   legal parameters and inputs.
3. Sequential and parallel engines are byte-for-byte equivalent.
4. New passwords receive fresh 16-byte salts and canonical PHC strings.
5. Verification is constant-time for equal-length tags and reports rehash needs.
6. Hostile encoded parameters are rejected under explicit limits before costly
   work begins.
7. Fuzz, concurrency, lint, documentation, and cross-endian checks pass.
8. The benchmark and security reports explain deployment trade-offs without
   claiming the implementation is production-ready.

## System-design practice

Treat the required implementation/conformance work as Senior scope. Its benchmark must state CPU,
memory, compiler profile, parameter set, concurrency, duration, and machine, and report p50/p95/p99
plus memory pressure. Staff practice adds a versioned password-policy service, online parameter and
PHC migration, bounded verification admission, compatibility, and rollback. Principal practice
defines organization-wide parameter governance, hardware/cost strategy, credential-incident
ownership, deprecation, and staged migration. The algorithm is fixed by the RFC; memory allocation,
parallel scheduling, API policy, operational limits, and migration remain design decisions.

## Extensions

- Implement Argon2i and Argon2d and run their RFC 9106 vectors.
- Support optional secret and associated-data inputs through an API that cannot
  accidentally serialize the secret.
- Create a policy calibrator that searches for the highest affordable memory and
  pass count under a latency target and concurrency budget.
- Compare a safe scalar implementation with reviewed SIMD backends.
- Add a migration utility that upgrades hashes after successful verification.
- Test interoperability with several language implementations using a shared
  vector corpus.
- Model authentication rate limiting and bounded hash-worker queues in a small
  service without exposing the educational core to real users.

## References

- [Annotated Argon2 implementation research](../../../research/argon2-implementation-guidance.md)
- [RFC 9106 — Argon2](https://www.rfc-editor.org/rfc/rfc9106.html)
- [RFC 7693 — BLAKE2](https://www.rfc-editor.org/rfc/rfc7693.html)
- [Argon2 paper](https://eprint.iacr.org/2015/430)
- [Argon2 reference implementation and full specification](https://github.com/P-H-C/phc-winner-argon2)
- [RFC 9106 verified erratum 7721](https://www.rfc-editor.org/errata/eid7721)
- [C2SP PHC Strings specification](https://c2sp.org/phc-strings)
- [Pinned Argon2 reference revision and KAT corpus](https://github.com/P-H-C/phc-winner-argon2/tree/f57e61e19229e23c4445b85494dbf7c07de721cb)
- [Password Hashing Competition](https://www.password-hashing.net/)
- [RustCrypto `argon2`](https://github.com/RustCrypto/password-hashes/tree/master/argon2)
- [RustCrypto `password-hash`](https://docs.rs/password-hash/latest/password_hash/)
- [RustCrypto `blake2`](https://docs.rs/blake2/latest/blake2/)
- [`subtle` constant-time primitives](https://docs.rs/subtle/latest/subtle/)
- [`zeroize` secret-erasure API](https://docs.rs/zeroize/latest/zeroize/)
- [NIST SP 800-63B](https://pages.nist.gov/800-63-4/sp800-63b.html)
- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [libsodium password-hashing documentation](https://doc.libsodium.org/password_hashing/default_phf)
- [IETF password-storage draft status](https://datatracker.ietf.org/doc/draft-ietf-kitten-password-storage/)
