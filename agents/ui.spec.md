# UI Spec

## Status
Implemented (TUI + CLI text output for `tinydew status`).

## TUI
- Header shows `tinydew day <day> <weather_icon> <time>` (same style as `tinydew status` top line).
- Map renders emoji tiles for current location.
- Player (`🧑`) marker is region-aware.
- Bottom message is prefixed as `> <message>`.
- Single compact controls line is shown:
  - `move: ↑↓←→ | clear: [C] | plant: [P] | water: [W] | harvest: [H] | trade: [T]`
- Legacy verbose control/status footer lines are not shown.

## CLI `status` layout
- Plain-text output (no TUI) includes:
  - Top line formatted as `tinydew day <day> <weather_icon> <time>` (example: `tinydew day 1 ☀️ 06:20`)
  - Map block (no map title label)
  - Optional plain inventory item lines (only when non-empty; no `Inventory` section header)
  - Money line formatted as `Money: 💰 $<amount>`
  - Bottom message line
- `status` intentionally omits dedicated location/player coordinate text blocks (map is authoritative).

## Festival UI Rules
- Spring Day 28 default bottom line:
  `Today is Butterfly Festival, enjoy it!`

## Rendering Constraints
- Map dimensions and icons follow active location map definition.
- The view reflects the player's current region and position.
- Wonder/Fountain/river/boundary visuals use tile emoji mapping from world tile types.
