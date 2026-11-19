# 02 — Systems programming: Minirun container runtime

## Project brief

Minirun is a deliberately small Linux container runtime: a tool that starts a program so it sees a private filesystem, hostname, process tree, and network stack. Build it to learn the kernel primitives beneath tools such as Docker, not to reproduce Docker's registry, build system, daemon, or orchestration features.

The runtime accepts an unpacked root filesystem and a command, creates an isolated container, applies resource and security controls, and supervises the container's PID 1 until exit. Target recent Linux kernels with cgroup v2 and document which operations require privileges or subordinate UID/GID mappings.

> Minirun is not a strong security boundary until user namespaces, capability dropping, `no_new_privs`, mount restrictions, and seccomp are implemented and tested. The CLI must warn when it runs with weaker isolation.

## Learning goals

- Create and join Linux namespaces using `clone3`, `unshare`, or well-audited wrappers.
- Build an isolated mount tree and switch roots without leaking host mounts.
- Understand why PID 1 must forward signals and reap orphaned descendants.
- Configure a network namespace, veth pair, bridge, routes, and loopback device.
- Enforce CPU, memory, process, and I/O limits with cgroup v2.
- Cross an `exec` boundary without leaking file descriptors, privileges, or host state.
- Design cleanup and recovery for partially created kernel resources.

## Scope

The core runtime supports one Linux host, one process container, an unpacked rootfs, and a small declarative configuration. It does not need Dockerfiles, layered image storage, registries, an OCI daemon, multi-host networking, orchestration, or cross-platform containers.

Implementing an OCI Runtime Specification subset is an optional compatibility goal. First make the lifecycle and isolation invariants correct with a smaller typed configuration.

## Required commands

| Command | Purpose |
| --- | --- |
| `minirun run <id> --rootfs <dir> -- <cmd> [args...]` | Create, start, attach to, and delete an ephemeral container |
| `minirun create <id> --bundle <dir>` | Validate a bundle and create a stopped container |
| `minirun start <id>` | Start the configured container process |
| `minirun state <id>` | Print lifecycle, PID, timestamps, namespaces, and exit information as JSON |
| `minirun list` | List locally known containers without trusting stale PID files |
| `minirun exec <id> -- <cmd> [args...]` | Join a running container's namespaces and execute a process |
| `minirun kill <id> [signal]` | Signal the container's init process through a verified process handle |
| `minirun delete <id>` | Remove a stopped container and all owned resources |
| `minirun doctor` | Check kernel, cgroup, namespace, mapping, and networking prerequisites |

Default to attached execution. Support `--detach`, `--env`, `--cwd`, `--hostname`, read-only bind mounts, resource limits, and `--network none|private`. Machine-readable output goes to `stdout`; diagnostics go to `stderr`.

## Bundle format

```text
my-container/
├── config.toml
└── rootfs/
    ├── bin/
    ├── etc/
    ├── proc/
    └── tmp/
```

```toml
schema = 1
hostname = "demo"
command = ["/bin/sh", "-c", "echo hello; sleep 30"]
cwd = "/"
environment = ["PATH=/bin:/usr/bin"]
readonly_rootfs = true

[namespaces]
user = true
mount = true
pid = true
uts = true
ipc = true
network = true

[resources]
memory_max = "128M"
pids_max = 64
cpu_max = "50000 100000"

[[mounts]]
source = "./data"
destination = "/data"
kind = "bind"
readonly = true
```

Reject unknown fields, relative in-container destinations, duplicate mounts, invalid cgroup values, host paths escaping allowed roots, and configurations that request unsupported isolation.

## Container creation sequence

Treat creation as a staged transaction with a cleanup action registered after every successful step:

1. Validate the ID, bundle, rootfs ownership, executable, and full configuration.
2. Reserve a private runtime directory such as `/run/user/<uid>/minirun/<id>` and lock the container ID.
3. Create a synchronization socket between the parent runtime and child bootstrap process.
4. Create user, mount, PID, UTS, IPC, and network namespaces. The child must wait while the parent writes UID/GID mappings.
5. Create a cgroup v2 subtree, apply limits, and place the child in it before workload execution.
6. Mark mount propagation private, construct the mount tree, bind the rootfs to itself, call `pivot_root`, detach the old root, and mount a fresh `/proc`.
7. Configure loopback and, for private networking, move one end of a veth pair into the network namespace and assign routes.
8. Set hostname, working directory, umask, environment, and terminal or standard streams.
9. Drop supplementary groups and capabilities, set `no_new_privs`, install seccomp, and close unintended file descriptors.
10. Start the container init, publish `Running` state atomically, forward signals, reap descendants, and record the final exit status.

If any step fails, unwind only resources owned by this creation attempt. Never delete a host interface, mount, cgroup, or directory based only on an unverified name.

## Isolation requirements

### Filesystem

- Use a mount namespace with private propagation so container mounts cannot affect the host.
- Prefer `pivot_root` to `chroot`; `chroot` alone is not an isolation boundary.
- Mount a namespace-correct `/proc` only after entering the new PID namespace.
- Apply bind-mount read-only flags with a second remount where required and verify the result from mount information.
- Reject symlink traversal and mount destinations that escape the new root.
- Mask or provide minimal versions of sensitive paths such as `/proc/sys`, `/proc/sysrq-trigger`, `/sys`, and host device nodes.

### Processes

- The first child inside the PID namespace becomes PID 1 and must reap orphaned descendants.
- Forward terminal and shutdown signals to the workload process group while keeping the init shim alive long enough to reap it.
- Use pidfds where supported to prevent PID-reuse mistakes in `state`, `kill`, and cleanup operations.
- On shutdown, send the configured graceful signal, wait on a monotonic deadline, then terminate the remaining cgroup.

### Network

- `none` creates a private namespace with loopback down by default.
- `private` enables loopback and creates a veth pair connected to a dedicated Minirun bridge.
- Allocate addresses under a lock from a local pool and release them idempotently.
- Configure routes directly through netlink rather than invoking shell command strings.
- Make outbound NAT an explicit opt-in because it changes host firewall state. Tag every rule so cleanup can prove ownership.

### Resources and security

- Require cgroup v2 and apply memory, CPU, PIDs, and optional I/O controls before exec.
- Prefer rootless user namespaces. Map the invoking user to container root and support subordinate ranges when configured.
- Drop all capabilities, then add only explicitly allowed capabilities after validating the namespace context.
- Set `no_new_privs`, provide a default seccomp allowlist, and reject setuid binaries when policy forbids them.
- Never pass the host environment implicitly. Always construct the child environment from configured values and safe defaults.

## Runtime state and recovery

Persist durable configuration under `${XDG_DATA_HOME}/minirun/containers/<id>` and live state under `${XDG_RUNTIME_DIR}/minirun/<id>`. A state record includes the runtime's schema version, lifecycle state, verified host PID identity, cgroup path, namespace handles, network allocation, owner UID, and timestamps.

Model `Creating`, `Created`, `Running`, `Stopping`, `Stopped`, and `Deleting` explicitly. State changes use a temporary file, flush, and atomic rename. On startup, reconcile recorded state with pidfds, namespace handles, cgroup membership, and owned network resources. A PID file by itself is never proof that a process belongs to Minirun.

## Suggested Rust architecture

```text
src/
├── cli/          # parsing, human output, JSON output
├── config/       # bundle schema and semantic validation
├── lifecycle/    # state machine, transactions, rollback, recovery
├── linux/        # namespaces, mounts, pidfds, netlink, cgroups, seccomp
├── init/         # container PID 1 signal forwarding and child reaping
├── network/      # bridge, veth, address allocation, routes, NAT ownership
└── state/        # durable and live state repositories
```

Keep raw syscalls and any `unsafe` blocks inside small Linux adapter modules with safe typed interfaces. Do not spread namespace file descriptors, raw PIDs, mount paths, or cgroup paths through command-handling code.

## Delivery stages

1. **Process container:** lifecycle state machine, UTS/PID/mount namespaces, rootfs switch, fresh `/proc`, PID 1 shim, and attached execution.
2. **Resource container:** user namespaces, rootless mappings, cgroup v2 limits, capability dropping, and recovery.
3. **Network container:** network namespace, loopback, veth, bridge, routes, address allocation, and owned cleanup.
4. **Secure runtime:** read-only mounts, `no_new_privs`, seccomp, masked paths, detached mode, `exec`, and fault-injection hardening.

## Required tests

### Unit and model tests

- `T01` — Every permitted lifecycle transition succeeds and every forbidden transition fails.
- `T02` — Bundle validation rejects path traversal, duplicate mounts, invalid limits, and unsupported isolation.
- `T03` — Each creation step registers the correct inverse cleanup action.
- `T04` — Address allocation is deterministic, exclusive, and reusable after release.

### Namespace and integration tests

- `T05` — The container sees its configured hostname while the host hostname is unchanged.
- `T06` — Container PID 1 differs from the host PID and reaps an orphaned grandchild without zombies.
- `T07` — Mounts and files created inside the container do not appear in the host mount namespace.
- `T08` — The rootfs cannot escape through `..`, symlinks, old-root handles, or bind destinations.
- `T09` — A private network has its own interfaces, address, route, and working loopback.
- `T10` — Memory, CPU, and process limits are enforced by the expected cgroup.
- `T11` — Signals reach the workload process group and its exit status reaches the caller.
- `T12` — `exec` joins the intended namespaces and refuses a stopped or identity-mismatched container.

### Security and resilience tests

- `T13` — An unprivileged container cannot mount, create host devices, load kernel modules, or change the host network.
- `T14` — Seccomp blocks a selected forbidden syscall and reports the failure clearly.
- `T15` — Killing Minirun after every creation step leaves no leaked mount, process, cgroup, veth, address, or state lock after recovery.
- `T16` — Reused host PIDs cannot cause `kill` or `delete` to target an unrelated process.
- `T17` — Fuzz bundle parsing, state reconciliation, and netlink message handling.
- `T18` — Run privileged and rootless test matrices on fresh Linux VMs in CI.

## Acceptance criteria

A developer can unpack a minimal rootfs, run a shell with a private hostname, mount tree, PID tree, network namespace, and cgroup limits, then inspect, signal, and delete it. The host remains unchanged outside explicitly owned runtime resources. Forced failure at any creation stage is recoverable and leaves no orphaned processes or kernel resources.

## Optional extensions

Add an OCI bundle adapter, overlayfs layers, rootless networking through `slirp4netns` or `pasta`, checkpoint and restore with CRIU, port forwarding, signed rootfs archives, or a shim API for a higher-level container manager.
