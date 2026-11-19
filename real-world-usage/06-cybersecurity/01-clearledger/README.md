# 01 — Cybersecurity: ClearLedger secret-scanning platform

## Company and service

ClearLedger is a fictional financial-software company. It needs to prevent credentials, private keys, and high-confidence tokens from entering source repositories or build artifacts.

Build `sentinelscan`, a defensive scanning platform with a local CLI, CI mode, and central findings API. The scanner must report secrets without storing or printing their full values.

## Brown Rust Book prerequisites

Study [strings and maps](https://rust-book.cs.brown.edu/ch08-00-common-collections.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [CLI I/O](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html), [iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), and [async](https://rust-book.cs.brown.edu/ch17-00-async-await.html).

## Required components

- `sentinelscan`: local and CI CLI.
- `scanner-core`: streaming decoders, rules, entropy checks, allowlists, and redaction.
- `sentinel-api`: authenticated ingestion and triage API.
- `sentinel-worker`: queued repository and artifact scans.
- `rulepack`: versioned signed detection rules.

## CLI interface

| Command | Purpose |
| --- | --- |
| `sentinelscan path <dir>` | Scan working-tree files without uploading contents |
| `sentinelscan git --since <ref>` | Scan added lines and reachable history in scope |
| `sentinelscan archive <file>` | Stream supported archives with extraction limits |
| `sentinelscan verify-rules <pack>` | Verify signature, schema, and compatibility |
| `sentinelscan baseline create` | Record reviewed fingerprints, never raw secrets |
| `sentinelscan ci --format sarif` | Produce CI and code-scanning results |

## Central API

| Method | Endpoint | Purpose |
| --- | --- | --- |
| `POST` | `/v1/scans` | Create a scan record and upload redacted findings |
| `GET` | `/v1/scans/{id}` | Read scan progress and summary |
| `GET` | `/v1/findings` | Filter findings by repository, rule, status, or age |
| `POST` | `/v1/findings/{id}/triage` | Mark confirmed, false positive, mitigated, or accepted |
| `POST` | `/v1/rulepacks` | Publish an authorized signed rule pack |
| `GET` | `/v1/rulepacks/latest` | Download the current compatible rule pack |

## Finding model

Store repository ID, commit, path, line, rule ID, severity, redacted preview, keyed fingerprint, confidence evidence, status, and timestamps.

Never persist raw matched values. Use a tenant-specific keyed fingerprint so repeated exposure can be correlated without making offline guessing easy.

## Detection behavior

- Combine exact formats, structural validation, entropy, contextual keywords, and denylisted known test values.
- Decode only bounded supported encodings and archives.
- Skip binaries by policy while allowing explicit overrides.
- Respect `.gitignore`, scanner exclusions, and centrally prohibited exclusions.
- Treat allowlist entries as reviewed fingerprints with expiry and owner.
- Redact findings in terminal, logs, API payloads, and crash reports.

## Example result

```text
HIGH AWS_ACCESS_KEY src/config.rs:18
preview: AKIA…7Q2M
fingerprint: fp_3f9c…
action: rotate the credential, remove it from history, then rescan
```

## Security requirements

- Threat-model malicious repositories, archives, regex denial of service, path traversal, symlink escape, decompression bombs, and terminal escape sequences.
- Use time and size budgets per file and archive.
- Verify signed rule packs before activation.
- Authenticate service calls and isolate tenant findings.
- Audit triage and rule changes.
- Provide a credential-rotation response guide; never claim deletion alone revokes a leaked secret.

## Required tests

### Unit tests

- `T01` — Each rule detects valid fixtures and rejects near misses.
- `T02` — Redaction never includes the complete matched value.
- `T03` — Fingerprints are stable within a tenant and differ across tenant keys.
- `T04` — Allowlist expiry and scope rules are enforced.

### Integration tests

- `T05` — Scan a fixture repository and emit expected SARIF locations.
- `T06` — Archive traversal and decompression-limit attacks are rejected safely.
- `T07` — Rule-pack signature failure leaves the previous rules active.
- `T08` — Central API tenant isolation prevents cross-tenant finding access.
- `T09` — Logs and error reports contain no raw fixture secrets.

### Quality, performance, and resilience tests

- `T10` — Measure precision and recall against a versioned labeled corpus.
- `T11` — Fuzz decoders, archive handling, rule parsing, and path normalization.
- `T12` — Scan a representative large repository within CPU and memory budgets.
- `T13` — Interrupt and resume a queued scan without duplicate findings.

## Local deployment and production design exercise

Build signed CLI artifacts and a minimal non-root server image. Run API and workers locally with
separate identities, encrypted test storage, egress controls, queue limits, and audited rule
publication; demonstrate backup, restore, key rotation, and rollback.

Provide security advisories, dependency policy, incident response, and tenant-deletion runbooks.
Production namespace and credential topology is a threat-model/design exercise unless executed.

## Acceptance criteria

The platform blocks confirmed high-severity fixtures in CI, keeps raw secrets out of storage and logs, resists malicious inputs, produces actionable locations, and supports audited triage without cross-tenant leakage.

## System-design practice

Declare repository/file/archive distributions, rule count, tenant mix, match prevalence, and
CPU/memory/I/O limits. Measure scan p95/p99, throughput, recall/precision, queue age, and saturation.
Senior delivers safe local and central scanning; Staff adds tenant fairness, resumable distributed
workers, signed rule compatibility, false-positive feedback, and staged rule rollout; Principal
defines enterprise ownership, retention, regional policy, cost, incident authority, and migration.
Use [OWASP threat modeling](https://owasp.org/www-community/Threat_Modeling) and
[NIST SSDF](https://csrc.nist.gov/Projects/ssdf) to review—not replace—the adversarial tests.

## Optional extensions

Add pre-receive hooks, IDE diagnostics, secret-provider validation using opt-in network calls, or historical remediation reports.
