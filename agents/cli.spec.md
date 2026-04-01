# TinyDew CLI Specification

## Overview

A command-line interface for TinyDew that allows users to interact with the game world through simple commands.

## Usage

```
tinydew [OPTIONS] <COMMAND> [ARGS...]
```

## Global Options

| Option | Description |
|--------|-------------|
| `-h`, `--help` | Display help information and exit |
| `-V`, `--version` | Display version information and exit |

## Commands

### `status`

Show the current UI/game status.

**Usage:**
```
tinydew status
```

### `do`

Execute an action in the game world.

**Usage:**
```
tinydew do <ACTION> [ARGS...]
```

**Supported Actions:**

| Action | Description | Examples |
|--------|-------------|----------|
| `water` | Water a specific area | `do water up`, `do water down`, `do water left`, `do water right` |
| `move` | Move character in a direction | `do move left`, `do move right`, `do move up`, `do move down` |
| `buy` | Purchase items | `do buy seed` |
| `sell` | Sell items | `do sell 🍓`, `do sell 🍄` |
| `fish` | Fish at a position | `do fish up`, `do fish left` |
| `clear` | Clear debris at a position | `do clear up`, `do clear left` |
| `plant` | Plant seeds at a position | `do plant up`, `do plant left` |
| `harvest` | Harvest at a position | `do harvest up`, `do harvest left` |
| `sleep` | Sleep through the night | `do sleep` |
| `play` | Play a piano note | `do play C4`, `do play D3`, `do play A5` |

## Examples

```bash
# Show help
tinydew -h

# Show version
tinydew -V

# Check status
tinydew status

# Water the area above
tinydew do water up

# Move left
tinydew do move left

# Buy seeds
tinydew do buy seed

# Sell strawberries
tinydew do sell 🍓

# Sell mushrooms
tinydew do sell 🍄

# Fish at up position
tinydew do fish up

# Clear up position
tinydew do clear up

# Plant at up position
tinydew do plant up

# Harvest at up position
tinydew do harvest up

# Play piano notes
tinydew do play C4
tinydew do play D4
tinydew do play A5
tinydew do play G3
```

## Notes

- This is a draft specification. Implementation details may vary.
- Directional commands (`up`, `down`, `left`, `right`) may need to map to game-specific coordinates.
- The `do` command parser should be extensible for future actions.
- Piano notes work in CLI mode via `do play <note>` — player must be at Farm `(4,3)` (directly south of the piano tile at `(4,2)`).
- If player is not at piano position, the action returns: `"Not near the piano."`
- Supported notes: C3, D3, E3, F3, G3, A3, B3, C4, D4, E4, F4, G4, A4, B4, C5, D5, E5, F5, G5, A5, B5.
