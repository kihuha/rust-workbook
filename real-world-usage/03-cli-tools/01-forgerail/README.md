# 01 — CLI tools: ForgeRail release automation

## Company and service

ForgeRail is a fictional developer-platform company with a monorepo containing Rust crates, JavaScript packages, and container services. Releases are slow because engineers manually determine changed packages and run inconsistent commands.

Build `releasectl`, a cross-platform CLI that calculates an affected dependency graph, validates release metadata, creates a reproducible plan, and executes approved release steps.

## Brown Rust Book prerequisites

Study [modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), [errors](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [tests](https://rust-book.cs.brown.edu/ch11-00-testing.html), the [CLI project](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html), [iterators and closures](https://rust-book.cs.brown.edu/ch13-00-functional-features.html), and [workspaces](https://rust-book.cs.brown.edu/ch14-03-cargo-workspaces.html).

## Required commands

| Command | Purpose |
| --- | --- |
| `releasectl detect --base <ref>` | List changed and transitively affected packages |
| `releasectl plan --base <ref> --out plan.json` | Produce an immutable, reviewable plan |
| `releasectl validate plan.json` | Validate versions, changelogs, dependencies, and credentials |
| `releasectl apply plan.json` | Execute an approved plan |
| `releasectl resume <run-id>` | Continue after an interrupted safe checkpoint |
| `releasectl status <run-id>` | Show step results and artifact locations |
| `releasectl completion <shell>` | Generate shell completion |

## Configuration

Read `release.toml` from the repository root. Define packages, dependency edges, version source, build command, test command, publish command, artifact patterns, and environment requirements.

Validate configuration before executing any command. Reject dependency cycles, duplicate package names, paths escaping the repository, and unknown step types.

## Plan format

```json
{
  "schema": 1,
  "base": "v2.4.0",
  "head": "abc123",
  "packages": [
    {"name":"api-core","from":"2.4.0","to":"2.5.0","reason":"changed"},
    {"name":"api-cli","from":"2.4.0","to":"2.5.0","reason":"depends_on:api-core"}
  ]
}
```

Hash the canonical plan. `apply` must verify that the current commit and plan hash still match. Default to `--dry-run`; require explicit confirmation or `--yes` in CI.

## Execution rules

- Topologically order packages and steps.
- Run independent build and test steps concurrently within a limit.
- Serialize publish steps when the registry requires it.
- Capture bounded stdout and stderr per step.
- Stop dependents after failure while allowing unrelated work to finish.
- Write an append-only run journal after each checkpoint.
- Never repeat a completed non-idempotent publish step during resume.

## User experience

- Provide useful exit codes for config, validation, build, test, publish, and internal failures.
- Support human text and `--format json` output.
- Respect `NO_COLOR` and non-interactive terminals.
- Print errors to `stderr` and machine-readable results to `stdout`.
- Include `--verbose`, but redact credentials and sensitive environment values.

## Required tests

### Unit tests

- `T01` — Changed packages expand to all transitive dependents.
- `T02` — Topological ordering rejects a dependency cycle with its path.
- `T03` — Version rules produce the expected semantic version bump.
- `T04` — Plan canonicalization produces a stable hash.

### Integration tests

- `T05` — A fixture monorepo produces the documented release plan.
- `T06` — Dry-run executes no external build or publish process.
- `T07` — Failed tests block only dependent publish steps.
- `T08` — Resume skips an already journaled publish checkpoint.
- `T09` — JSON output remains valid when a child process writes arbitrary text.

### Cross-platform and operational tests

- `T10` — Run the supported command matrix on Linux, macOS, and Windows CI.
- `T11` — Interrupt a run between checkpoints and resume without duplication.
- `T12` — Verify Ctrl-C terminates child processes and writes a recoverable journal.

## Distribution and production use

Publish versioned binaries with checksums and provenance attestations. Support `cargo install`, package-manager formulas, and direct archives. Pin CI to exact versions and document compatibility guarantees for `release.toml` and plan schemas.

Provide a threat model covering malicious repository content, command injection, untrusted paths, symlinks, credential exposure, and compromised artifacts.

## Acceptance criteria

An engineer can clone a fixture repository, generate and review a plan, execute it in dry-run, apply it, interrupt it, resume safely, and inspect machine-readable results without manual state repair.

## Optional extensions

Add signed approvals, remote build workers, registry adapters, release pull requests, or software-bill-of-materials generation.
