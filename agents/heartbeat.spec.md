# Heartbeat / Pause-State Spec

## Status
Implemented.

## Behavior
- Runtime tracks pause-like state fields surfaced in TUI/`status` summaries where applicable.
- Heartbeat-related status is included in the same state views the player already sees (not a separate protocol).
- Core gameplay remains deterministic and command-driven.
