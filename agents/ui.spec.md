# UI Spec

## Status
Implemented (CLI text output only for `tinydew status` — no interactive/TUI mode).

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
