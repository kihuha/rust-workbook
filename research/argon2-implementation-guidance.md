# Argon2id implementation and password-storage guidance

Research date: 2026-11-19

## Scope and source policy

This note supports the ArgonForge cybersecurity brief. It separates three
questions that should not be answered by the same source:

1. **What bytes must a conforming Argon2id v1.3 implementation produce?**
2. **How should those bytes and parameters be serialized and tested?**
3. **How should an application choose and operate a password-hashing policy?**

The first question is governed by RFC 9106. The second is best answered by the
PHC string specification, the Argon2 reference implementation, and independent
implementations. The third requires current operational guidance and local
measurement; it cannot be frozen to one parameter tuple forever.

This note favors specifications, original papers, source repositories,
maintainers' documentation, and government or standards-body guidance. It does
not treat tutorials or general-purpose blog posts as implementation authority.

## Recommended source hierarchy

Use sources in this order when they disagree:

1. **Algorithm conformance:** [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html)
   plus the RFC Editor's [verified errata](https://www.rfc-editor.org/errata/eid7721).
2. **BLAKE2b primitive:** [RFC 7693](https://www.rfc-editor.org/rfc/rfc7693.html).
3. **Password-hash serialization:** the stable [C2SP PHC Strings specification](https://c2sp.org/phc-strings).
4. **Known-answer oracle:** the fixed
   [reference Argon2 repository revision](https://github.com/P-H-C/phc-winner-argon2/tree/f57e61e19229e23c4445b85494dbf7c07de721cb)
   and its [Argon2id KAT corpus](https://github.com/P-H-C/phc-winner-argon2/blob/f57e61e19229e23c4445b85494dbf7c07de721cb/kats/argon2id).
5. **Independent Rust oracle and API comparison:** the maintained
   [RustCrypto `argon2` documentation](https://docs.rs/argon2/latest/argon2/)
   and [source repository](https://github.com/RustCrypto/password-hashes/tree/master/argon2).
6. **Deployment policy:** benchmark locally, then compare the result with
   [RFC 9106 Section 4](https://www.rfc-editor.org/rfc/rfc9106.html#section-4),
   the [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html),
   [libsodium password-hashing guidance](https://doc.libsodium.org/password_hashing/default_phf),
   and applicable organizational or regulatory requirements.

The original paper and current standards work belong beside this hierarchy as
design and operational context, not above the published RFC for byte-level
conformance.

## Primary-source map

| Source | What it establishes | How ArgonForge should use it |
| --- | --- | --- |
| [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html) | Argon2 v1.3 inputs, `H₀`, `H'`, memory geometry, indexing, compression, permutation, recommended profiles, security discussion, and official vectors | Normative algorithm checklist and required intermediate/final tests |
| [RFC 9106 verified erratum 7721](https://www.rfc-editor.org/errata/eid7721) | Corrects “raging” to “ranging” in Section 3.2; it does not change the algorithm | Link beside the RFC and teach learners to check errata before implementation |
| [RFC 7693](https://www.rfc-editor.org/rfc/rfc7693.html) | BLAKE2b parameters, byte order, compression, test material, and reference C | Validate the ordinary BLAKE2b primitive used by `H₀` and `H'`; do not substitute it for Argon2's BlaMka-based block permutation |
| [Argon2 v1.3 specification](https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf) and [original paper](https://eprint.iacr.org/2015/430) | Design rationale, attack analysis, variants, and the evolution of the construction | Explain why the algorithm has its memory-access and trade-off properties; use RFC 9106 when notation or version details differ |
| [Reference C implementation](https://github.com/P-H-C/phc-winner-argon2) | Authors' implementation, CLI/library behavior, encoding code, KAT files, and low-level control flow | Differential oracle and ambiguity resolver, preferably at a pinned revision; do not transliterate it line by line |
| [C2SP PHC Strings](https://c2sp.org/phc-strings) | Deterministic `$id$...` grammar, parameter rules, decimal rules, unpadded Base64, salt/output guidance, and parser duties | Specify canonical output and strict bounded parsing independently from the core algorithm |
| [RustCrypto `argon2`](https://docs.rs/argon2/latest/argon2/) | Maintained pure-Rust implementation and high-level PHC hash/verify APIs | Independent differential oracle and model for Rust-facing API boundaries; never call it from the student algorithm core |
| [RustCrypto `password-hash`](https://docs.rs/password-hash/latest/password_hash/) and [`phc`](https://docs.rs/phc/latest/phc/) | Generic password-hashing traits and PHC data types | Compare parsing, random-salt, and verification design; pin compatible crate versions because the APIs evolve |
| [RustCrypto PHC `Output`](https://github.com/RustCrypto/formats/blob/master/phc/src/output.rs) | A maintained Rust example of bounded output and constant-time equality | Review the project's tag type and verification tests against this design without copying it blindly |
| [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html) | Current application-oriented minimums, salts, peppers, tuning, migration, and denial-of-service concerns | Operational review checklist, not an algorithm specification |
| [NIST SP 800-63B-4, password verifiers](https://pages.nist.gov/800-63-4/sp800-63b.html#passwordver) | Salted password hashing, practical cost factors, stored algorithm/cost metadata, evolution over time, and optional separately stored keyed protection | Support policy versioning and rehash-on-login; do not claim that this section specifically standardizes Argon2 |
| [libsodium password hashing](https://doc.libsodium.org/password_hashing/default_phf) and [Argon2id constants](https://github.com/jedisct1/libsodium/blob/master/src/libsodium/include/sodium/crypto_pwhash_argon2id.h) | A production API with hash, verify, rehash, raw derivation, fixed salt size, and named resource profiles | API and operations comparison; its profiles are implementation policy, not RFC conformance rules |
| [IETF password-storage draft 13](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-13.html) | Active Best Current Practice work for password storage, salts, peppers, algorithm choice, migration, and denial-of-service limits | Watch for future guidance and discuss it explicitly as an expiring Internet-Draft, not a finished standard |

## Algorithm-conformance findings

### RFC 9106 is the byte-level contract

RFC 9106 corresponds to Argon2 version 1.3 and says every implementation of the
document must support Argon2id. It defines the complete input domain, including
password `P`, salt `S`, lanes `p`, tag length `T`, requested memory `m`, passes
`t`, version `v`, optional secret `K`, associated data `X`, and type `y`.
ArgonForge should keep these names in its specification notes so code and tests
can be traced back to the RFC without translation ambiguity
([RFC 9106 Sections 1 and 3.1](https://www.rfc-editor.org/rfc/rfc9106.html#section-3.1)).

The RFC distinguishes requested memory `m` from effective memory `m'`, which is
rounded down to a multiple of `4p`. The requested value `m`, not the rounded
value, is included in `H₀`. This deserves a dedicated regression test because an
implementation can allocate the correct number of blocks while still commit to
the wrong parameter in its initial digest
([RFC 9106 Section 3.2](https://www.rfc-editor.org/rfc/rfc9106.html#section-3.2)).

Argon2id generates addresses like Argon2i only during pass zero, slices zero and
one; it uses Argon2d-style data-dependent addressing afterward. The four slices
are synchronization boundaries: same-slice segments in different lanes can be
computed in parallel, but legal reference regions exclude blocks that are not
yet available. These statements should become branch and schedule tests rather
than remain prose
([RFC 9106 Sections 3.4.1.3 and 3.4.2](https://www.rfc-editor.org/rfc/rfc9106.html#section-3.4.1.3)).

The official Argon2id vector supplies more than the final tag: it includes the
pre-hashing digest and first/last words after each pass. ArgonForge should stop
at each checkpoint while the implementation is being built. A final-tag-only
failure localizes nothing
([RFC 9106 Section 5.3](https://www.rfc-editor.org/rfc/rfc9106.html#section-5.3)).

The RFC Editor currently lists one verified erratum for RFC 9106. Erratum 7721
fixes an editorial typo (“raging” to “ranging”) in the description of later
passes and does not alter the formulas. Even so, the brief should link the
errata record to establish a reusable standards-reading habit
([RFC 9106 erratum 7721](https://www.rfc-editor.org/errata/eid7721)).

### BLAKE2b and Argon2's permutation are related but not interchangeable

RFC 7693 specifies ordinary BLAKE2b, including its little-endian word
interpretation and reference material. Argon2 uses BLAKE2b for its initial hash
and variable-length construction, but its 1024-byte compression function uses a
BLAKE2-derived permutation with the additional low-32-bit multiplication term.
Tests should therefore be split between the external BLAKE2b primitive, `H'`,
BlaMka/`GB`, the 16-word permutation, and full-block compression
([RFC 7693 Sections 2–3](https://www.rfc-editor.org/rfc/rfc7693.html#section-2),
[RFC 9106 Sections 3.3 and 3.5–3.6](https://www.rfc-editor.org/rfc/rfc9106.html#section-3.3)).

The original Argon2 paper and the repository's v1.3 specification explain the
memory-hard design and time-space trade-off analysis. They are valuable for the
learner's security rationale, but RFC 9106 is newer as an implementer-oriented
description and explicitly identifies itself as version 1.3. Resolve byte-level
questions in favor of the RFC and its errata
([Argon2 paper](https://eprint.iacr.org/2015/430),
[Argon2 v1.3 specification](https://github.com/P-H-C/phc-winner-argon2/blob/master/argon2-specs.pdf),
[RFC 9106 introduction](https://www.rfc-editor.org/rfc/rfc9106.html#section-1)).

## Serialization and parser findings

The old PHC organization repository now points readers to the C2SP specification.
Use the stable C2SP page rather than treating the moved repository file as the
canonical text
([PHC repository move notice](https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md),
[C2SP PHC Strings](https://c2sp.org/phc-strings)).

PHC Strings require deterministic producer encoding. The grammar orders the
algorithm identifier, optional version, ordered parameters, salt, and output.
Parameter names cannot be duplicated, and each algorithm must define its
allowed parameter order and ranges. ArgonForge should emit one canonical form,
while deciding and documenting whether its consumer accepts any non-canonical
forms permitted by the generic specification
([C2SP PHC Strings, Specification and Function Duties](https://c2sp.org/phc-strings#specification)).

PHC B64 uses the RFC 4648 Base64 alphabet without `=` padding or whitespace.
The specification also calls out invalid non-zero trailing bits. Strict parser
tests should cover length modulo four, padding, whitespace, alphabet errors,
and non-canonical trailing bits rather than relying on a generic permissive
Base64 decoder
([C2SP PHC Strings, B64](https://c2sp.org/phc-strings#b64)).

PHC decimal encoding has no leading zeroes, uses ASCII digits, and has a unique
encoding for each non-negative version or Argon2 cost value. Rejecting signed,
overflowing, empty, and non-canonical numbers before allocation is both an
interoperability and resource-safety requirement
([C2SP PHC Strings, Decimal Encoding](https://c2sp.org/phc-strings#decimal-encoding)).

The PHC guidance recommends a 16-byte random salt and a 32-byte output, while
RFC 9106 recommends a 16-byte salt and says 16 output bytes are sufficient for
most applications. ArgonForge's 16-byte salt and 32-byte default tag are
therefore conservative and interoperable, while the raw algorithm API may
support the broader RFC tag range
([C2SP PHC Strings, Function Duties](https://c2sp.org/phc-strings#function-duties),
[RFC 9106 Sections 3.1 and 4](https://www.rfc-editor.org/rfc/rfc9106.html#section-4)).

## Implementation-oracle findings

### Reference C implementation

The authors' repository contains separate files for core orchestration,
reference compression/filling, thread handling, and text encoding. Those are
useful review lenses for module boundaries and ambiguous index arithmetic:

- [`src/core.c`](https://github.com/P-H-C/phc-winner-argon2/blob/f57e61e19229e23c4445b85494dbf7c07de721cb/src/core.c)
  covers validation, initialization, finalization, and memory handling.
- [`src/ref.c`](https://github.com/P-H-C/phc-winner-argon2/blob/f57e61e19229e23c4445b85494dbf7c07de721cb/src/ref.c)
  contains the portable reference block-filling and indexing path.
- [`src/thread.c`](https://github.com/P-H-C/phc-winner-argon2/blob/f57e61e19229e23c4445b85494dbf7c07de721cb/src/thread.c)
  exposes platform threading concerns.
- [`src/encoding.c`](https://github.com/P-H-C/phc-winner-argon2/blob/f57e61e19229e23c4445b85494dbf7c07de721cb/src/encoding.c)
  implements Argon2 text encoding and decoding.
- [`kats/argon2id`](https://github.com/P-H-C/phc-winner-argon2/blob/f57e61e19229e23c4445b85494dbf7c07de721cb/kats/argon2id)
  supplies a larger known-answer corpus than the single RFC example.

The pinned revision above is the repository's current `master` revision found
during this research. Pinning makes fixtures reproducible even if the branch
moves. The project should still record the revision in generated fixture
metadata and should never copy C pointer arithmetic into unsafe Rust without a
new Rust-specific soundness argument.

### RustCrypto implementation

RustCrypto's `argon2` crate is a pure-Rust implementation supporting Argon2d,
Argon2i, and Argon2id, with Argon2id v19 as its default. Its public examples
separate raw output from PHC-string hashing and verification, making it a useful
comparison for ArgonForge's core/password-layer boundary
([RustCrypto `argon2` documentation](https://docs.rs/argon2/latest/argon2/),
[RustCrypto Argon2 source](https://github.com/RustCrypto/password-hashes/tree/master/argon2/src)).

The current RustCrypto source exposes recommended salt length 16, explicit
parameter validation, optional zeroization, external-memory APIs, and separate
initialization/filling/finalization paths. Its source tests include
[known-answer tests](https://github.com/RustCrypto/password-hashes/blob/master/argon2/tests/kat.rs)
and [PHC string tests](https://github.com/RustCrypto/password-hashes/blob/master/argon2/tests/phc_strings.rs).
These are useful independent fixtures, but agreement between two implementations
is evidence, not a proof of correctness
([RustCrypto Argon2 library source](https://github.com/RustCrypto/password-hashes/blob/master/argon2/src/lib.rs),
[parameter source](https://github.com/RustCrypto/password-hashes/blob/master/argon2/src/params.rs)).

The maintained RustCrypto PHC `Output` type bounds decoded output length and
uses constant-time equality for comparisons. ArgonForge may use a reviewed
constant-time primitive rather than inventing one, but it should still test that
the verification path reaches that comparison only after safe parsing and
bounded derivation
([RustCrypto PHC `Output` source](https://github.com/RustCrypto/formats/blob/master/phc/src/output.rs)).

RustCrypto's repository and `latest` docs can change independently of a learner's
lockfile. Differential tests should record the crate version and features, and
CI should pin them. A second oracle such as the reference C CLI prevents a
single moving dependency from silently redefining expected output.

## Deployment-policy findings and conflicts

Parameter recommendations answer different operational questions. They do not
change Argon2's byte-level definition.

| Guidance current on 2026-11-19 | Argon2id profile or rule | Interpretation |
| --- | --- | --- |
| [RFC 9106 Section 4](https://www.rfc-editor.org/rfc/rfc9106.html#section-4) | First profile: 2 GiB, `t=1`, `p=4`; lower-memory profile: 64 MiB, `t=3`, `p=4`; both use 16-byte salt and 32-byte tag | General CFRG profiles plus a procedure that starts from affordable memory and time |
| [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id) | Listed minimum trade-offs include 46 MiB/`t=1`/`p=1` and 19 MiB/`t=2`/`p=1` | Application-oriented minimums; benchmark and raise them where the service budget allows |
| [RustCrypto current parameter source](https://github.com/RustCrypto/password-hashes/blob/master/argon2/src/params.rs) | Default is 19 MiB, `t=2`, `p=1`, 32-byte output | Library default currently aligns with an OWASP-listed minimum, not with either RFC uniform profile |
| [libsodium Argon2id constants](https://github.com/jedisct1/libsodium/blob/master/src/libsodium/include/sodium/crypto_pwhash_argon2id.h) | Interactive: 64 MiB/`t=2`; moderate: 256 MiB/`t=3`; sensitive: 1 GiB/`t=4` | Named implementation profiles designed for different resource budgets; the high-level API also exposes `needs_rehash` |
| [IETF draft 13 Section 5.1](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-13.html#section-5.1) | Draft table lists 2 GiB, `t=2`, `p=1`, 32-byte output | Useful signal from active standards work, but not final; the draft expires 2027-02-08 and can change |

The correct curriculum conclusion is not to pick a winner from this table.
ArgonForge should distinguish:

- a **conformance profile** small enough for deterministic tests;
- a **documented example policy** that is safe to run on typical learner
  hardware;
- a **deployment policy** selected by latency, peak-memory, concurrency, and
  abuse-budget measurements on the actual target; and
- **verification limits** that cap values read from untrusted stored strings.

RFC 9106 explicitly tells implementers to determine affordable memory and time,
then tune passes. OWASP likewise warns that excessive work factors can degrade
performance and contribute to denial of service. The brief should require load
tests at realistic authentication concurrency, not only a single-hash benchmark
([RFC 9106 Section 4](https://www.rfc-editor.org/rfc/rfc9106.html#section-4),
[OWASP Work Factors](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#using-work-factors)).

NIST SP 800-63B-4 requires a suitable salted password-hashing scheme, says the
cost factor should be as high as practical without harming verifier performance,
and says it should increase over time. It also calls for storing scheme and cost
metadata to enable migration. The cited section does not name Argon2, so it
supports ArgonForge's policy-versioning and rehash design but must not be quoted
as NIST approval of this specific algorithm
([NIST SP 800-63B-4 Section 3.1.1.2](https://pages.nist.gov/800-63-4/sp800-63b.html#passwordver)).

NIST additionally recommends a separate keyed operation with a key stored apart
from password hashes, preferably in hardware-protected storage. OWASP describes
peppering with similar separation and rotation concerns. This is not the same
as a per-password salt, and it is an application architecture decision. If
ArgonForge explores Argon2's optional secret `K`, its PHC record must never
contain the secret and its documentation must explain recovery and rotation
([NIST SP 800-63B-4 Section 3.1.1.2](https://pages.nist.gov/800-63-4/sp800-63b.html#passwordver),
[OWASP Peppering](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#peppering)).

The active IETF password-storage document is an Internet-Draft with intended
Best Current Practice status, not a published RFC. Version 13 contains useful
salt, pepper, migration, DoS, and Argon2 guidance, but it expires on 2027-02-08.
The research note or brief should link the unversioned datatracker page for
future review while citing version 13 for any exact claim
([datatracker history](https://datatracker.ietf.org/doc/draft-ietf-kitten-password-storage/),
[draft 13](https://www.ietf.org/archive/id/draft-ietf-kitten-password-storage-13.html)).

## Testing and assurance implications

The expanded source set implies the following test structure:

1. Test the selected BLAKE2b implementation against RFC 7693 material before
   using it in `H₀` or `H'`
   ([RFC 7693 appendices](https://www.rfc-editor.org/rfc/rfc7693.html#appendix-A)).
2. Test `H₀`, pass checkpoints, and the final tag against RFC 9106 Section 5.3
   ([RFC vector](https://www.rfc-editor.org/rfc/rfc9106.html#section-5.3)).
3. Import versioned Argon2id KAT fixtures from the reference repository and
   retain their provenance and checksum
   ([reference KAT](https://github.com/P-H-C/phc-winner-argon2/blob/f57e61e19229e23c4445b85494dbf7c07de721cb/kats/argon2id)).
4. Differentially generate low-cost cases with both the pinned reference CLI
   and a pinned RustCrypto crate version; compare raw bytes, not only encoded
   strings
   ([reference CLI](https://github.com/P-H-C/phc-winner-argon2#command-line-utility),
   [RustCrypto raw API](https://docs.rs/argon2/latest/argon2/struct.Argon2.html#method.hash_password_into)).
5. Give PHC parsing its own canonicalization and rejection corpus derived from
   the C2SP grammar, B64, and decimal sections
   ([C2SP PHC Strings](https://c2sp.org/phc-strings)).
6. Test verification limits with hostile encoded costs before any large
   allocation or worker creation; test the current policy separately from the
   stored policy
   ([OWASP work-factor warning](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#using-work-factors)).
7. Treat Miri, fuzzing, sanitizers, race detection, and zeroization review as
   complementary evidence. No one technique proves a cryptographic
   implementation suitable for production.

Known-answer and differential tests must run with deliberately cheap parameters
in ordinary CI. Deployment-sized benchmarks belong in a separate job with
explicit memory and concurrency controls; otherwise a correct security test can
become a CI availability problem.

## Concrete changes recommended for the ArgonForge brief

1. Add RFC 7693 and the verified RFC 9106 erratum to the normative reading
   list.
2. Replace any generic “PHC format” implication with a direct link to the stable
   C2SP PHC Strings specification.
3. Add the pinned reference KAT corpus and RustCrypto tests as named fixture
   sources.
4. Require fixture metadata to record source URL, revision/version, variant,
   version, parameters, and checksum.
5. State explicitly that deployment profiles conflict by design and that RFC,
   OWASP, RustCrypto, libsodium, and an Internet-Draft serve different roles.
6. Keep algorithm-valid ranges separate from lower application verification
   caps.
7. Add a test for strict PHC Base64 trailing bits and canonical decimal values.
8. Add a test that the requested `m`, rather than rounded `m'`, enters `H₀`.
9. Use both a pinned C oracle and a pinned Rust oracle for differential tests.
10. Add NIST-backed policy migration language without claiming NIST-specific
    endorsement of Argon2.
11. Treat peppering as a separately managed keyed architecture with rotation,
    not as a hidden or reused salt.
12. Add a scheduled source-review task because cost guidance and draft standards
    evolve faster than the algorithm specification.

## Suggested learner reading order

1. Read RFC 9106 Sections 1–3 once for the whole data flow.
2. Read RFC 7693 only deeply enough to validate the BLAKE2b primitive boundary.
3. Implement and test `H'`, block compression, initialization, addressing,
   filling, and finalization while repeatedly returning to the corresponding RFC
   section.
4. Use the RFC vector, then the reference KAT corpus, then two differential
   implementations.
5. Read C2SP PHC Strings before writing the text parser or encoder.
6. Read OWASP, NIST, libsodium, and the current IETF draft only after the raw
   algorithm is correct, then design policy, verification limits, migration,
   and rehash behavior.
7. Finish by reading the original paper/specification for rationale and attack
   analysis; do not make the paper the sole implementation checklist.

## Maintenance notes

- Recheck the [RFC 9106 errata search](https://www.rfc-editor.org/errata_search.php?rfc=9106)
  before each course release.
- Pin Git commits for copied fixtures and Cargo versions for differential
  oracles.
- Revisit the OWASP, RustCrypto, and libsodium profiles when dependencies or
  deployment hardware change.
- Recheck the [IETF draft status](https://datatracker.ietf.org/doc/draft-ietf-kitten-password-storage/)
  after 2027-02-08 or when it advances to an RFC.
- Keep the research date visible. Operational recommendations age; RFC 9106
  conformance does not silently change with library defaults.
