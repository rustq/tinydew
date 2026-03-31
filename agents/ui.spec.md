# UI Spec

## Status
Implemented (interactive TUI + MCP text snapshot UI).

## Interactive UI
- Header shows `tinydew day <day> <weather_icon> <time>` (same style as MCP snapshot top line).
- Map renders emoji tiles for current location.
- Player (`🧑`) and guest (`👧`) markers are region-aware.
- Bottom message is prefixed as `> <message>`.
- Single compact controls line is shown:
  - Guest: `move: ↑↓←→ | greet: [SPACE]`
  - Player: `move: ↑↓←→ | clear: [C] | plant: [P] | water: [W] | harvest: [H] | trade: [T]`
- Legacy verbose control/status footer lines are not shown.

## MCP `print` UI
- Text snapshot includes:
  - Top line formatted as `tinydew day <day> <weather_icon> <time>` (example: `tinydew day 1 ☀️ 06:20`)
  - Map block (no map title label)
  - Optional plain inventory item lines (only when non-empty; no `Inventory` section header)
  - Money line formatted as `Money: 💰 $<amount>`
  - Bottom message line
- MCP snapshot intentionally omits location/player/guest position text blocks.

## Festival UI Rules
- Spring Day 28 default bottom line:
  `Today is Butterfly Festival, enjoy it!`
- Guest festival greet override line:
  `✨ Happy Butterfly Festival!`

## Rendering Constraints
- Map dimensions and icons follow active location map definition.
- Non-active entity markers are hidden from unrelated map contexts.
- Wonder/Fountain/river/boundary visuals use tile emoji mapping from world tile types.
