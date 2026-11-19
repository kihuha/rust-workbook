# 02 — Cloud infrastructure: Service health and endpoint monitor

## What this teaches

Cloud systems need small, reliable agents that run continuously, read declarative configuration, perform work on a schedule, and turn raw observations into operational signals. Build `watchtower`, a local monitoring daemon that checks configured URLs and reports when their health changes.

This is an infrastructure process, not a web API. Operate it through a YAML or TOML file, command-line flags, logs, signals, and exit behavior.

## Build

The program must:

- load named endpoints, check intervals, timeouts, and expected status codes from YAML or TOML;
- schedule checks independently so one slow endpoint does not delay the others;
- bound concurrent network work;
- record timestamp, endpoint, status code, response time, and error category as JSON Lines or structured text;
- track consecutive successes and failures rather than alerting on one transient error;
- emit a `DOWN` alert only when an endpoint crosses its failure threshold;
- emit a `RECOVERED` alert only when it becomes healthy again;
- reload valid configuration on `SIGHUP` or a file change without restarting; and
- stop cleanly on Ctrl-C, allowing in-flight checks a bounded time to finish.

Start with terminal alerts. Add a webhook, email, or chat integration only as an extension; the lesson is health-state evaluation, not building another service.

## Example configuration

```yaml
defaults:
  interval_seconds: 5
  timeout_ms: 1500
  failure_threshold: 3
  recovery_threshold: 2

endpoints:
  - name: documentation
    url: https://example.com/
    expected_status: 200
  - name: local-test-service
    url: http://127.0.0.1:9000/health
    expected_status: 204
```

## Suggested milestones

1. Check one URL once and print its status and latency.
2. Deserialize a configuration file and check every entry.
3. Run checks repeatedly with independent schedules and timeouts.
4. Persist observations and model `Unknown`, `Healthy`, and `Down` states.
5. Add threshold-based down and recovery alerts.
6. Add config reload, graceful shutdown, and bounded concurrency.

## Rust and infrastructure concepts

- Tokio tasks, intervals, cancellation, and timeouts.
- Serde configuration and validation.
- State machines for health transitions.
- Structured logging and basic metrics.
- Backpressure with a semaphore.
- Unix signals and graceful process lifecycle.
- The difference between a probe result, health state, an alert, and an SLO.

## Failure experiments

Use local dummy servers to introduce delayed responses, `500` responses, refused connections, malformed configuration, and intermittent recovery. Observe how thresholds prevent flapping and how timeouts keep one target from consuming all capacity.

## Required tests

- A timeout, connection error, unexpected status, and success are classified separately.
- Checks overlap without exceeding the configured concurrency limit.
- Exactly one `DOWN` event is emitted when the failure threshold is crossed.
- Exactly one `RECOVERED` event is emitted after the recovery threshold is crossed.
- A bad reload leaves the last valid configuration active.
- Shutdown completes without corrupting the observation log.

## Done when

Run the daemon against at least three local endpoints, break and recover them independently, and use the logs to explain exactly when and why each health transition occurred.

## Extensions

Add exponential retry backoff, rolling uptime windows, Prometheus output, DNS timing, TLS-expiry checks, container packaging, or a systemd unit.
