# Non-Interactive / MCP-Driven Play Spec

## Status
Implemented.

## Behavior
- Game can be advanced entirely via MCP `command` / `commandBatch` without TUI input.
- `print` provides textual UI snapshot for automation.
- State and map endpoints support external orchestration.
- Semantics match core gameplay rules used by interactive mode.
