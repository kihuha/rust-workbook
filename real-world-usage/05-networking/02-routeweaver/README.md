# 02 — Networking: RouteWeaver link-state router

## What this teaches

Routers at the network layer must choose where to forward packets. That choice
can change when a link becomes slow, a router fails, or a better path becomes
available.

Build `routeweaver`, a command-line link-state routing simulator. It will read
an extensive network topology from a text file, calculate the best route
between two nodes, and print every hop in that route.

The project simulates the routing decision rather than transmitting real
packets. Its graph represents the topology information that a link-state
routing protocol has already learned.

## Algorithm: Dijkstra's shortest-path first

Use **Dijkstra's algorithm** with link latency as the edge weight.

This is the appropriate algorithm because:

- the router has a graph of the network;
- every link has a non-negative latency;
- the desired route has the lowest total latency, not necessarily the fewest
  hops; and
- an adjacency list and binary heap can process large sparse graphs in
  `O((V + E) log V)` time.

This is the shortest-path-first idea used by link-state routing protocols such
as OSPF and IS-IS. Do not use breadth-first search: it minimizes hop count and
ignores latency. Bellman-Ford supports negative weights, which this model does
not need. A* requires a meaningful heuristic that the topology does not
provide.

## Required behavior

The program must:

1. Accept a topology file, source node, and destination node on the command
   line.
2. Parse nodes, bidirectional links, and link latencies from the file.
3. Allow a node to be marked down in the file.
4. Exclude a failed node and all of its links from routing decisions.
5. Find the available path with the lowest total latency.
6. Print the ordered node hops and total latency.
7. Report useful errors for malformed files, unknown nodes, failed endpoints,
   and unreachable destinations.
8. Read the file on every invocation so that editing it changes the next
   routing decision without recompiling.

## Suggested topology format

Design a plain-text format that remains comfortable to edit by hand. One
possible format is:

```text
# Links are bidirectional. Latency is in milliseconds.
link A B 4
link B C 2
link C F 3

link A D 5
link D E 3
link E F 4

# Uncomment to simulate a failed router.
# down B
```

You may choose another format, such as TOML, if you document it clearly. The
important requirement is that a learner can easily change a latency or node
status and rerun the program.

## Command-line interface

Aim for an invocation like:

```bash
cargo run -- topology.txt A F
```

With the example topology, the initial result should be:

```text
Path: A -> B -> C -> F
Total latency: 9 ms
```

After marking `B` down, the result should change to:

```text
Path: A -> D -> E -> F
Total latency: 12 ms
```

Restore `B`, then change the `B`–`C` latency from `2` to `20`. The route should
again use `A -> D -> E -> F` because that path now has the lower total latency.

## Suggested data model

Think through these responsibilities before writing the algorithm:

- A node needs an identifier and an operational state.
- A link connects two nodes and has a non-negative latency.
- The topology needs efficient access to every outgoing link for a node.
- A route contains ordered hops and total latency.
- The search needs a tentative distance and possible predecessor for each
  node.

An adjacency list is a good representation for a large, sparse network. Avoid
an adjacency matrix unless you can justify its `O(V²)` memory cost.

## Milestones

### 1. Parse and validate the topology

Read the file, ignore comments and blank lines, build the graph, and reject
invalid directives or latency values with line-numbered errors.

### 2. Explore the graph

Given a node, print its directly connected neighbors and link costs. This
small checkpoint helps verify the graph before implementing routing.

### 3. Implement shortest-path first

Implement Dijkstra's algorithm. Keep algorithm code separate from file parsing
and CLI output so each part can be tested independently.

### 4. Reconstruct the route

Finding the minimum total is not enough. Track predecessors during the search,
then walk backward from the destination and reverse the result to produce the
ordered hops.

### 5. Model failures

Exclude down nodes while exploring neighbors. Decide what should happen when
the source or destination itself is down and return an explicit error.

### 6. Handle operational errors

Cover unknown nodes, disconnected graphs, invalid input, unreadable files,
integer overflow, and incorrect CLI arguments without panicking.

## Required tests

- The lowest-latency path is chosen even when it has more hops.
- The reported total equals the sum of the selected link latencies.
- Marking an intermediate node down selects an alternate path.
- Increasing a link's latency can select an alternate path.
- A down source or destination is rejected.
- Disconnected nodes produce a clear “no route” result.
- Source equal to destination returns a zero-latency, one-node route.
- An unknown node produces a useful error.
- Malformed directives and invalid latencies report the input line.
- Cycles and zero-latency links do not cause an infinite loop.

Generate at least one larger topology to check that the implementation remains
responsive and does not use memory proportional to `V²`.

## Done when

The program reads a topology file and prints the lowest-latency route between
two CLI-supplied nodes. Manually failing a node or increasing a link latency in
the file causes the next invocation to choose the correct alternate path. The
routing algorithm, parser, and failure cases are covered by automated tests.

## System-design practice

Declare node/link count, update rate, latency distribution, churn, message loss, and CPU/memory
limits. Measure convergence p95/p99, route-computation time, message volume, stale-route duration,
and saturation. Senior simulates correct shortest paths; Staff implements a bounded link-state
protocol with sequence numbers, flooding, failure detection, and rolling compatibility; Principal
defines areas/hierarchy, policy ownership, incident behavior, capacity, cost, and migration. Use
[RFC 2328](https://www.rfc-editor.org/rfc/rfc2328) as protocol research while keeping the project a
small educational model rather than an OSPF clone.

## Extensions

- Support directed links or different latency in each direction.
- Find several loop-free alternate paths rather than only the best path.
- Add bandwidth, loss, or administrative cost to a composite route metric.
- Compare Dijkstra using a binary heap with a simple `O(V²)` implementation.
- Watch the topology file and recompute routes when it changes.
- Print the full shortest-path tree from one source.
- Export the topology and selected route in Graphviz DOT format.
