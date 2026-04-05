# TinyDew CLI Specification

## Overview

TinyDew provides a **CLI interface only** — no interactive or TUI mode. All interaction with the game world happens through command-line commands.

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

Execute an action in the game world. Does not print status output — only returns an action result message.

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
| `sell` | Sell items | `do sell 🍓`, `do sell 🍄`, `do sell 🌺` |
| `fish` | Fish at a position | `do fish up`, `do fish left` |
| `clear` | Clear debris at a position | `do clear up`, `do clear left` |
| `plant` | Plant seeds at a position | `do plant up`, `do plant left` |
| `harvest` | Harvest at a position | `do harvest up`, `do harvest left` |
| `sleep` | Sleep through the night | `do sleep` |

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

# Sell flowers
tinydew do sell 🌺

# Fish at up position
tinydew do fish up

# Clear up position
tinydew do clear up

# Plant at up position
tinydew do plant up

# Harvest at up position
tinydew do harvest up
```

## Notes

- This is a draft specification. Implementation details may vary.
- Directional commands (`up`, `down`, `left`, `right`) may need to map to game-specific coordinates.
- The `do` command parser should be extensible for future actions.
