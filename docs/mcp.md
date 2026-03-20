# Tinydew MCP Server

## Starting the MCP Server

To run Tinydew as an MCP server process:

```bash
tinydew --mcp
```

This starts the server using stdio transport for local-only communication.

## Protocol

The MCP server accepts JSON-RPC style requests via stdin and responds via stdout.

### Available Methods

- `startSession` - Create a new game session
- `endSession` - Close a session
- `getState` - Get structured game state
- `getMap` - Get current map view
- `getStats` - Get session statistics
- `command` - Execute a single command
- `commandBatch` - Execute multiple commands

### Example Request

```json
{"method": "startSession", "params": {"seed": 42, "mode": "standard"}}
```

### Example Response

```json
{"ok":true,"result":{"session_id":"...","initial_state":{...}}}
```