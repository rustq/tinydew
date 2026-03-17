# MCP Implementation Tasks

Objective: implement Shelldew MCP support with stable session, command, query, and error contracts for automation clients.

## Execution Order

1. [01-mcp-server-skeleton.task.md](./01-mcp-server-skeleton.task.md) — MCP server wiring, tool/resource registration
2. [02-session-lifecycle.task.md](./02-session-lifecycle.task.md) — session create/close and validation
3. [03-command-tools.task.md](./03-command-tools.task.md) — single/batch command execution
4. [04-query-tools-and-resources.task.md](./04-query-tools-and-resources.task.md) — state/map/stats tools + resources
5. [05-error-safety-and-limits.task.md](./05-error-safety-and-limits.task.md) — error contract, safety defaults, limits
6. [06-tests-verification-and-docs.task.md](./06-tests-verification-and-docs.task.md) — tests, verification, usage docs

## MCP Usage Guide

Shelldew exposes a Model Context Protocol (MCP) interface that allows developers to drive the game programmatically. This guide covers how to use Shelldew via MCP clients.

### Available Tools

| Tool | Description |
|------|-------------|
| `shelldew.start_session` | Create a new game session |
| `shelldew.command` | Execute a single gameplay command |
| `shelldew.command_batch` | Execute multiple commands in sequence |
| `shelldew.get_state` | Get current structured game state |
| `shelldew.get_map` | Get current map view |
| `shelldew.get_stats` | Get game statistics |
| `shelldew.end_session` | Close a session |

### Available Resources

| URI | Description |
|-----|-------------|
| `shelldew://session/{session_id}/state` | Game state snapshot |
| `shelldew://session/{session_id}/map` | Current map view |
| `shelldew://session/{session_id}/inventory` | Player inventory |
| `shelldew://session/{session_id}/log/recent` | Recent log entries |

### Quick Start Example

```python
# Python example using MCP client
from mcp.client import Client

client = Client("shelldew-server")

# Start a new session
result = client.call_tool("shelldew.start_session", {
    "seed": 42,
    "mode": "standard"
})
session_id = result["session_id"]

# Execute commands
client.call_tool("shelldew.command", {
    "session_id": session_id,
    "command": "move:down"
})

client.call_tool("shelldew.command", {
    "session_id": session_id,
    "command": "buy:carrot:5"
})

# Get current state
state = client.call_tool("shelldew.get_state", {
    "session_id": session_id
})
print(f"Day: {state['day']}, Money: ${state['money']}")

# End session
client.call_tool("shelldew.end_session", {
    "session_id": session_id
})
```

### Command Reference

All commands follow the format `command:argument` (case-insensitive):

| Command | Format | Description |
|---------|--------|-------------|
| Movement | `move:up\|down\|left\|right` | Move player |
| Clear | `clear` | Clear current tile |
| Plant | `plant:carrot\|strawberry\|cauliflower\|rhubarb` | Plant a crop |
| Water | `water` | Water current tile |
| Harvest | `harvest` | Harvest from current tile |
| Buy | `buy:<crop>[:<qty>]` | Buy seeds (default qty: 1) |
| Sell | `sell:<crop>[:<qty>]` | Sell produce (default qty: 1) |
| Sleep | `sleep` | Sleep to advance day |
| Print | `print` | Get text snapshot of game |

### Batch Commands

Execute multiple commands in one call:

```python
result = client.call_tool("shelldew.command_batch", {
    "session_id": session_id,
    "commands": [
        "move:down",
        "plant:carrot",
        "water",
        "move:right"
    ],
    "stop_on_error": False  # Continue even if a command fails
})
```

### Error Handling

Errors follow this format:

```json
{
  "ok": false,
  "error": {
    "code": "INVALID_COMMAND",
    "message": "Unknown command 'fly'. Valid commands: ...",
    "details": { "valid": ["move:up", "move:down", ...] }
  }
}
```

Error codes:
- `INVALID_COMMAND` - Unknown command
- `VALIDATION_ERROR` - Invalid arguments
- `SESSION_NOT_FOUND` - Session ID not found
- `SESSION_CLOSED` - Session already closed
- `INTERNAL_ERROR` - Internal server error
- `NOT_IMPLEMENTED` - Feature not implemented

### Deterministic Seeding

Use the `seed` parameter when starting a session for reproducible gameplay:

```python
result = client.call_tool("shelldew.start_session", {
    "seed": 12345,  # Same seed = same game
    "mode": "standard"
})
```

### Using Resources

Read game data via MCP resources:

```python
# Read game state via resource
state = client.read_resource(f"shelldew://session/{session_id}/state")

# Read map
map_data = client.read_resource(f"shelldew://session/{session_id}/map")

# Read inventory
inventory = client.read_resource(f"shelldew://session/{session_id}/inventory")
```

### Session Limits

- Maximum 10 concurrent sessions
- Sessions auto-cleanup after 30 minutes of inactivity
- Always close sessions when done to free resources
