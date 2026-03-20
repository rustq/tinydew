# MCP Interface Spec

## Status
Implemented (stdio JSON-RPC style in main runtime fallback).

## Server
- MCP server runs over stdio.
- Requests are JSON lines with `method` + `params`.

## Methods
- `startSession`
- `endSession`
- `getState`
- `getMap`
- `getStats`
- `getWorldTime`
- `command`
- `commandBatch`

## Command Grammar (current)
- Movement: `move:up|down|left|right`
- Farming: `clear[:dir]`, `plant:<crop>[:dir]`, `water[:dir]`, `harvest[:dir]`
- Economy: `buy:<item>[:qty]`, `sell:<item>[:qty]`
- Fishing: `fishing[:dir]`, `sell:fish[:qty]`, `sell:rare[:qty]`
- Utility: `print`, `save`, `load`
- `sleep` is rejected in MCP API.

## Error Model
- Structured `ok/result/error` responses.
- Invalid commands/validation failures return machine-readable errors.

## Notes
- Movement commands route to guest movement when guest mode is enabled and active in state.
