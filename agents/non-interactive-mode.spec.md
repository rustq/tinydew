# Non-Interactive Mode Specification

## Overview

Enable Shelldew CLI to run in command-driven non-interactive mode for automation and CI workflows.

## Scope Decisions

- Keep mode flag name as `--batch`.
- No `--output` flag; no `quiet|json` output modes.
- Default runtime behavior is quiet (no per-step output).
- Output is text only.
- No script-file support (`--script`).
- No custom initial-state flags (`--day`, `--hour`, `--money`, etc.).

## CLI Input Parameters

### Core Flags

| Flag | Type | Description |
|------|------|-------------|
| `--batch`, `-b` | flag | Run in non-interactive mode |
| `--command <cmd>`, `-c <cmd>` | option (repeatable) | Execute one command per occurrence, in order |
| `--stats` | flag | Print final summary in text form at end of run |

## Command Format

Commands are passed as `<action>[:<target>]` (or extended forms shown below):

```text
move:<direction>             direction: up|down|left|right
clear                        clear tile in front
plant:<crop>                 crop required: carrot|strawberry|cauliflower|rhubarb
water                        water tile in front
harvest                      harvest tile in front
buy:<item>[:<qty>]           buy item with optional qty (default 1)
sell:<item>[:<qty>]          sell item with optional qty (default 1)
sleep                        sleep (advance to next day)
print                        print current state snapshot (text)
quit                         terminate remaining command execution early
```

Notes:
- `trade` is not allowed in non-interactive mode.
- Planting must always specify crop explicitly (no implicit current seed).

## Examples

```bash
# Single command
shelldew --batch --command "move:right"

# Multiple commands in sequence
shelldew -b -c "move:right" -c "clear" -c "plant:carrot" -c "water" -c "print"

# Buy/sell flow with final stats
shelldew -b -c "buy:carrot:4" -c "sell:carrot:1" --stats
```

## Validation Rules

### Batch Mode Requirements
- `--batch` must be present for non-interactive execution.
- Without `--batch`, CLI starts interactive TUI mode.

### Command Requirements
- At least one `--command` is required in batch mode.
- Commands execute strictly in provided order.
- Unknown command -> error.

### Command Validation
- Invalid direction example: `error: invalid direction 'north' (valid: up|down|left|right)`
- Invalid crop example: `error: invalid crop 'tomato' (valid: carrot|strawberry|cauliflower|rhubarb)`
- Invalid qty example: `error: invalid quantity '-1' (must be positive integer)`

### Error Handling / Exit Codes
- Success: exit code `0`.
- Any validation or execution error: exit code `1`.
- No additional exit-code classes are defined in this iteration.
- Errors are written to stderr.

## Output Behavior

- Default: no output while commands execute.
- `print` command: writes a text snapshot to stdout.
- `--stats`: writes final text summary to stdout.

## Minimal Stats Schema (for implementation contract)

Even though output is text-only, stats generation should follow this internal minimal schema:

- `day` (integer)
- `time` (string `HH:MM`)
- `location` (string)
- `money` (integer)
- `inventory` (map item->count)
- `status` (string; `ok` or `error`)

Text output for `--stats` can format these fields for humans while preserving the same underlying data model.
