# 02 — CLI tools: Iterator planning and issue tracker

## Project brief

Iterator is a local-first, Linear-inspired planning and issue tracker for teams that want their product data to live beside their code. Build a Rust CLI that manages initiatives, projects, cycles, triage, board views, and issues without a database or hosted service.

Every issue, document, template, and written status update is a Markdown document that can be read and edited with an ordinary text editor. Other durable records use TOML or immutable JSON event files. A generated index may make reads faster, but it must be disposable: deleting it and running `iterator rebuild` must recover the complete workspace from the source files.

The target is a CLI interpretation of Linear's core planning model, not a pixel-for-pixel clone. It should support the full path from strategic initiatives and projects to cycles, triage, issues, saved views, and delivery insights.

## Recommended storage design

Use **one file per durable record**, rather than one large configuration file. This produces smaller Git diffs, fewer merge conflicts, and simple atomic writes. Keep board membership derived from issue metadata so the same fact is never stored in two places.

```text
.iterator/
├── workspace.toml
├── teams/
│   └── engineering.toml
├── workflows/
│   └── engineering.toml
├── initiatives/
│   └── developer-experience.toml
├── projects/
│   └── rust-workbook.toml
├── milestones/
│   └── public-beta.toml
├── cycles/
│   └── engineering-2026-24.toml
├── views/
│   └── active-work.toml
├── routing/
│   └── support-bugs.toml
├── templates/
│   └── bug-report.md
├── issues/
│   └── engineering/
│       ├── ENG-0001.md
│       └── ENG-0002.md
├── docs/
│   └── projects/
│       └── rust-workbook-spec.md
├── updates/
│   └── rust-workbook/
│       └── 2026-11-19.md
├── events/                   # immutable history for metrics and audits
│   └── 2026-11/
│       └── 01J...K2.json
├── comments/                 # optional extension
│   └── ENG-0001/
│       └── 01J...K2.md
└── .cache/                   # generated; safe to delete and gitignore
    └── index.json
```

Use stable IDs for filenames; do not include titles in paths because renaming a title should not rename a file. Store dates as RFC 3339 UTC values and include `schema` on every record so future migrations are explicit.

### Issue format

Use TOML front matter for machine-readable fields and Markdown for the description. TOML keeps the entire workspace on one serialization format and is well supported by Serde.

```markdown
+++
schema = 1
id = "ENG-0001"
title = "Design the file storage boundary"
team = "engineering"
project = "rust-workbook"
status = "in_progress"
priority = "high"
assignees = ["darius"]
labels = ["architecture", "storage"]
estimate = 5
cycle = "engineering-2026-24"
milestone = "public-beta"
blocks = ["ENG-0002"]
related = ["ENG-0007"]
rank = "hzzzzz"
created_at = "2026-11-19T08:00:00Z"
updated_at = "2026-11-19T09:15:00Z"
revision = 3
+++

Define the repository API and ensure every write is atomic.
```

Store `parent` on a child issue and derive the parent's sub-issue list. Store `blocks`, `related`, and `duplicate_of` only on the source issue; inverse relations such as *blocked by* are derived. This avoids two files claiming different versions of the same relationship.

## Core feature requirements

### Issues

- Support nested sub-issues with their own status, assignees, estimate, and priority. Calculate parent progress from descendant estimates or issue counts; do not persist a second progress value.
- Support blocking, blocked-by, related, and duplicate relations with cycle detection for parent and blocking graphs.
- Store reusable issue templates as Markdown plus TOML front matter. A template may provide a title pattern, description, labels, priority, estimate, and default assignees.
- Configure either numeric, Fibonacci, or T-shirt estimates per team. Use the priorities `urgent`, `high`, `medium`, `low`, and `none`.

### Projects and roadmaps

- Group projects beneath initiatives and place projects on a month- or quarter-based roadmap.
- Model milestones as ordered project deliverables with target dates. Derive their progress from linked issues.
- Store project updates as dated Markdown files with structured `on_track`, `at_risk`, or `off_track` health in front matter.
- Store project dependencies in one direction and derive the inverse. Detect dependency cycles and surface schedule conflicts.

### Cycles

- Define a team cadence, duration, start day, and rollover policy. Materialize cycle TOML files so past schedules cannot change when the team configuration changes.
- Close cycles automatically when a command observes that their end time has passed. Move unfinished work according to the rollover policy while preserving its original-cycle event history.
- Derive velocity, burn-up, scope changes, and completion forecasts from immutable issue events. Git history alone is not sufficient because commits may be squashed or delayed.

### Triage

- Represent triage as an issue intake state separate from the backlog.
- Support accept, mark duplicate, snooze-until, and decline actions. Declined issues remain auditable instead of being deleted.
- Apply ordered routing rules based on source, team, labels, or text matches. A dry run must explain which rule matched and which fields it would change.

### Custom views and insights

- Save dynamic filters over assignee, label, priority, cycle, project, status, estimate, and dates. Render the same query as a list, grouped table, or terminal Kanban board.
- Mark views as personal or shared. Personal views still live in files but may be placed in an ignored local directory.
- Calculate lead time, cycle time, created-versus-resolved counts, velocity, scope growth, and assignee workload from source records and immutable events.

### Docs

- Store PRDs, technical specifications, and meeting notes as Markdown linked to a project or initiative through front matter.
- Create an issue from selected document lines or standard input, insert a stable issue link back into the document, and record the source document on the issue.

## Core records

| Record | Source of truth | Notes |
| --- | --- | --- |
| Workspace | `workspace.toml` | Schema version, defaults, and ID policy |
| Team | `teams/<slug>.toml` | Name, key such as `ENG`, and next issue number |
| Workflow | `workflows/<team>.toml` | Ordered statuses and allowed transitions |
| Initiative | `initiatives/<slug>.toml` | Strategic outcome, owner, target window, and project references |
| Project | `projects/<slug>.toml` | Initiative, lead, dates, and project dependencies; latest health is derived from updates |
| Milestone | `milestones/<slug>.toml` | Project deliverable, order, and target date |
| Cycle | `cycles/<slug>.toml` | Materialized cadence window and rollover state |
| Board view | `views/<slug>.toml` | Filters, grouping, sorting, and displayed fields |
| Routing rule | `routing/<slug>.toml` | Ordered triage predicates and assignment actions |
| Issue | `issues/<team>/<id>.md` | Metadata in front matter and description in Markdown |
| Template or doc | `templates/*.md`, `docs/**/*.md` | Structured front matter and editable Markdown content |
| Project update | `updates/<project>/<date>.md` | Health, author, date, and narrative update |
| Event | `events/<month>/<ulid>.json` | Immutable state transition used for history and insights |
| Search index | `.cache/index.json` | Derived data only; never authoritative |

TOML is recommended over a single JSON file because it is pleasant to edit by hand, maps cleanly to Rust types, and gives each record an independent Git history. A board file should store a query such as `status != "done"`, not a copied list of issue IDs. Manual board order belongs on each issue as a sortable `rank` value.

## Required commands

| Command | Purpose |
| --- | --- |
| `iterator init` | Create and validate a new `.iterator` workspace |
| `iterator issue create` | Create an issue from flags, a template, or an editor |
| `iterator issue show <id>` | Render one issue, its children, and relations |
| `iterator issue edit <id>` | Open the source document in `$EDITOR` and validate it afterward |
| `iterator issue list` | Filter, sort, and format matching issues |
| `iterator issue move <id> <status>` | Apply a valid workflow transition |
| `iterator template create|list|apply` | Manage and apply standardized issue templates |
| `iterator project create|list|show` | Manage project records, milestones, and dependencies |
| `iterator initiative create|list|show` | Manage strategic initiative records |
| `iterator roadmap` | Render initiatives, projects, milestones, and dependencies on a timeline |
| `iterator cycle start|close|status` | Operate scheduled cycles and rollover |
| `iterator triage list|accept|duplicate|snooze|decline` | Process the intake queue |
| `iterator board <view>` | Render a saved view grouped into terminal columns |
| `iterator insights [view]` | Calculate delivery and workload metrics |
| `iterator doc issue create <doc> --lines <range>` | Turn selected document text into an issue |
| `iterator validate` | Check schemas, references, IDs, and workflow states |
| `iterator rebuild` | Recreate all derived indexes from source files |

Support human-readable tables by default and stable `--format json` output for scripts. Mutating commands should offer `--dry-run`, and all errors and diagnostics should go to `stderr`.

## Storage and consistency rules

- Acquire a workspace lock for mutations. Write to a temporary file in the destination directory, flush it, then atomically rename it.
- Validate the complete new record before replacing the old one. Never leave a partially written canonical file.
- Reserve issue numbers under the lock. Gaps after a crash are acceptable; reusing an ID is not.
- Reject unknown teams, projects, statuses, and labels unless the command explicitly creates them.
- Use a `revision` field for optimistic concurrency. Refuse to overwrite a record changed since it was read.
- Treat a multi-file operation as a small transaction: stage every replacement, write a recovery manifest, then rename files into place. Recover or roll back the manifest on the next command.
- Preserve the Markdown body when updating front matter. A formatter may normalize metadata, but must not rewrite prose unnecessarily.
- Write an immutable event for every accepted state change. Use a ULID filename so parallel events do not contend for one append-only log.
- Make event creation and record replacement one recoverable multi-file operation. A record without its event would corrupt historical insights.
- Rebuild the cache when its schema, workspace fingerprint, or source-file modification data no longer matches.

Git is a collaboration mechanism, not a transaction manager. `iterator validate` should detect conflict markers, duplicate IDs, invalid references, and inconsistent team counters after merges, then print actionable repair instructions.

## Suggested Rust architecture

Keep command handling separate from persistence so a later TUI or server can reuse the same application logic.

```text
src/
├── cli/           # argument parsing and terminal rendering
├── domain/        # planning records, relations, schedules, IDs, invariants
├── application/   # issues, roadmap, cycles, triage, docs, and insights
├── storage/       # Markdown/TOML/JSON repositories, locking, atomic writes
└── index/         # disposable search and query acceleration
```

Define repository traits in the application boundary and implement them in `storage`. Domain types should not know about paths, Clap, terminal colors, or TOML syntax.

## Delivery stages

Build the core feature set incrementally so each stage remains usable:

1. **Foundation:** workspace validation, atomic storage, teams, workflows, issues, sub-issues, relations, templates, and saved views.
2. **Planning:** initiatives, projects, milestones, roadmaps, health updates, dependencies, and docs.
3. **Execution:** cycles, automatic rollover, triage actions, routing rules, and immutable events.
4. **Insights:** burn-up, velocity, lead time, cycle time, workload, and creation-versus-resolution reports.

Authentication, real-time collaboration, hosted synchronization, notifications, comments, attachments, and a graphical UI are outside the core project. Comments can be added later as one immutable Markdown file per comment; avoid appending them to an issue description because concurrent comments would then conflict with ordinary issue edits.

## Required tests

### Unit tests

- `T01` — Issue front matter round-trips without changing the Markdown body.
- `T02` — Workflow transitions reject unknown or disallowed states.
- `T03` — Parent progress is derived correctly from nested sub-issues.
- `T04` — Relation and project dependency cycles are rejected with their paths.
- `T05` — View filters and stable rank ordering produce the expected board columns.
- `T06` — Invalid IDs, cross-record references, and schema versions produce precise errors.
- `T07` — Routing rules are ordered, deterministic, and explain their matches.
- `T08` — Burn-up and lead-time calculations reproduce a known event fixture.

### Integration tests

- `T09` — A user can initialize a workspace, create a templated issue, move it, and see it on a board.
- `T10` — A project can be linked to an initiative, milestones, an update, and a dependency on the roadmap.
- `T11` — Closing a cycle rolls unfinished issues forward and preserves historical velocity.
- `T12` — Triage actions and routing rules produce the expected issue state and audit events.
- `T13` — Selected document lines create an issue with valid links in both directions.
- `T14` — Deleting `.cache` and rebuilding produces identical query and insight results.
- `T15` — Interrupting a write leaves the previous record valid and its event recoverable.
- `T16` — Concurrent edits are rejected instead of silently overwriting a newer revision.
- `T17` — JSON output remains valid while diagnostics are written to `stderr`.
- `T18` — Validation identifies conflict markers and duplicate IDs in a merged fixture.

## Acceptance criteria

In a Git repository, a developer can initialize Iterator, create planning records and issues, edit Markdown source files, run a cycle and triage workflow, inspect a roadmap, and render saved views and insights. Another clone can reproduce the same state using only tracked files. Removing every generated cache changes no durable data, and corrupt records are reported with file paths and field-level errors rather than being silently skipped.

## System-design practice

Declare issue/project counts, change frequency, concurrent writers, query mix, attachment sizes,
and local storage limits. Measure startup, mutation, search, rebuild, and sync p95/p99 as data grows.
Senior delivers a correct local source of truth; Staff adds multi-user synchronization, conflict
handling, migrations, audit, bounded background indexing, and offline recovery; Principal defines
tenant isolation, data residency, ownership, compatibility policy, cost, and migration to a hosted
service. Compare event log, relational state, and document files; keep the first implementation
simple enough that each guarantee remains visible.

## Optional extensions

Add a ratatui interface, comments, full-text indexing with Tantivy, Git-aware conflict helpers, import/export for Linear, editor plugins for highlighting-to-issue, or a small read-only web viewer. Any index or UI must continue to treat the Markdown, TOML, and immutable event files as the source of truth.
