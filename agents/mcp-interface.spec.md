# MCP Interface Spec

## Status
Implemented (stdio JSON-RPC style).

## Server
- MCP server runs over stdio in runtime fallback mode.
- Request format: JSON line with `method` + `params`.

## Methods
- `startSession`, `endSession`
- `getState`, `getMap`, `getStats`, `getWorldTime`
- `command`, `commandBatch`

## Commands (current)
- Move: `move:up|down|left|right`
- Farm: `clear[:dir]`, `plant:seed[:dir]`, `water[:dir]`, `harvest[:dir]`
- Trade: `buy:seed[:qty]`, `sell:<item>[:qty]`
- Fishing: `fish[:dir]`, `sell:fish[:qty]`, `sell:rare[:qty]`
- Utility: `print`, `save`, `load`
- `sleep` is intentionally rejected in MCP API.

## Non-Interactive Mode
- Full gameplay loop can be advanced via MCP commands without TUI key input.
- `print` gives automation-friendly UI snapshot text.
- MCP auto-resolves `HomeState::Income` to morning (`close_home`) so batch commands do not get stuck on sleep-income screen.
- State/map endpoints support external orchestration.
