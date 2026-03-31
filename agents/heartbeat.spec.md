# Heartbeat / Pause-State Spec

## Status
Implemented.

## Behavior
- Runtime tracks pause-like state fields used by MCP stats/state snapshots.
- Heartbeat-related status is surfaced in summary/state responses.
- Core gameplay remains deterministic and command-driven.
