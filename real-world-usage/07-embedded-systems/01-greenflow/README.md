# 01 — Embedded systems: GreenFlow irrigation controller

## Company and device

GreenFlow is a fictional agricultural technology company. It deploys solar-powered irrigation controllers where connectivity is intermittent and a stuck valve can damage crops or waste water.

Build firmware for a Cortex-M class microcontroller. The device reads soil-moisture and flow sensors, operates four valves, stores schedules, and publishes telemetry through a separate radio module.

## Brown Rust Book prerequisites

Study [ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html), [enums](https://rust-book.cs.brown.edu/ch06-00-enums.html), [error handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html), [traits](https://rust-book.cs.brown.edu/ch10-02-traits.html), [concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html), and [unsafe Rust](https://rust-book.cs.brown.edu/ch20-01-unsafe-rust.html).

## Hardware assumptions

- Four normally closed valves controlled through GPIO drivers.
- Soil-moisture ADC channels per irrigation zone.
- One pulse-based flow sensor.
- Real-time clock with backup power.
- Flash region for two configuration slots and an event journal.
- Independent watchdog.
- Radio coprocessor connected over UART or SPI.

Document the exact development board, MCU, HAL, clock, voltage, pin map, and memory budget. Keep hardware access behind traits so logic runs in host tests.

## Operating modes

- `BootSelfTest`: validate storage, clock, sensor range, and valve feedback.
- `Idle`: sample sensors and await schedule or command.
- `Watering`: open one permitted valve and monitor flow.
- `Fault`: close all valves, persist cause, and require safe recovery.
- `Maintenance`: allow authenticated local diagnostics without automatic watering.

## Control rules

- Never energize more than one valve unless hardware is explicitly rated for it.
- Close every valve on panic, watchdog reset, brownout detection, or lost task heartbeat.
- Stop watering when duration or volume limit is reached.
- Detect no-flow and unexpected-flow faults within configured windows.
- Apply sensor plausibility ranges and debouncing.
- Use monotonic time for durations and wall-clock time only for schedules.
- Persist configuration with version, checksum, and atomic A/B slots.

## Local command protocol

Use a framed, checksummed serial protocol with bounded messages.

```text
GET_STATUS -> mode=Idle, battery_mv=12410, faults=[]
SET_SCHEDULE zone=2 start=06:00 duration_s=600 -> ACK revision=18
OPEN_VALVE zone=2 max_s=60 maintenance_token=… -> ACK
```

Reject replayed maintenance commands using a nonce or sequence number. Never allow a remote command to bypass hard safety limits.

## Telemetry

Publish compact versioned records containing device ID, firmware version, boot ID, sequence, battery, sensor readings, valve state, flow, faults, and configuration revision.

Queue a bounded number of records during outages. When full, retain faults and recent summaries according to a documented policy.

## Required tests

### Host unit tests

- `T01` — Every invalid state transition closes all simulated valves.
- `T02` — Duration and volume limits stop watering at the exact boundary.
- `T03` — No-flow and unexpected-flow conditions enter `Fault`.
- `T04` — A/B configuration recovery selects the newest valid slot.
- `T05` — Sequence handling rejects replayed commands.

### Hardware-in-the-loop tests

- `T06` — ADC boundary values map to documented sensor states.
- `T07` — Watchdog reset closes physical outputs and records reset cause.
- `T08` — Brownout during configuration write preserves one valid slot.
- `T09` — Radio disconnect does not block the control loop.
- `T10` — Valve feedback mismatch raises a fault within the target time.

### Endurance and safety tests

- `T11` — Run a 7-day accelerated schedule without memory growth or task stalls.
- `T12` — Fuzz serial frames and verify bounded parsing with no unsafe output action.
- `T13` — Cycle power at every persistence checkpoint and recover consistently.
- `T14` — Verify worst-case execution time leaves the required control-loop margin.

## Firmware architecture

Use `no_std` where appropriate. Separate pure control logic, device drivers, persistence, protocol, telemetry, and board configuration. Confine unsafe register access to reviewed HAL or small documented wrappers.

Use a static task model with bounded queues and no heap allocation after startup unless the platform and failure policy explicitly permit it.

## Release and field operations

Produce reproducible signed firmware images with version and hardware compatibility metadata. Support staged update, integrity verification, rollback to a known-good image, and recovery through a physical maintenance interface.

Provide manufacturing test, calibration, device provisioning, key rotation, fault-code, safe-decommissioning, and field-recovery procedures.

## Acceptance criteria

The controller passes hardware safety tests, remains deterministic under communication loss, recovers from interrupted writes and power loss, never exceeds valve limits, and can be diagnosed and rolled back in the field.

## Optional extensions

Add rainfall input, adaptive scheduling, encrypted telemetry, low-power sleep modes, or a bootloader-based over-the-air update system.
