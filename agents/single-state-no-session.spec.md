# Single State / No Session Isolation Spec

## Status
Implemented.

## Behavior
- Tinydew runs with one authoritative game state.
- MCP `startSession` returns a compatibility id (`singleton`) and does not create isolated worlds.
- Multiple calls continue the same world state.
- `endSession` is a compatibility no-op.

## Persistence Coupling
- State is loaded from save on startup when available.
- State is autosaved by MCP flow after command batches and day-transition flows.

## Acceptance
- Progress remains across repeated MCP starts.
- No per-session world reset exists in runtime behavior.
