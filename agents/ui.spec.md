# UI Spec

## Status
Implemented (interactive TUI + MCP text snapshot UI).

## Interactive UI
- Header shows season, day, weather/time, and control context.
- Map renders emoji tiles for current location.
- Player (`🧑`) and guest (`👧`) markers are region-aware.
- Bottom message area shows current contextual text.
- Controls hint line is shown (guest vs player control variants).

## MCP `print` UI
- Text snapshot includes:
  - Day/time/weather header
  - Location
  - Money
  - Player block
  - Optional guest block (when guest active/visible by current rules)
  - Inventory block
  - Map block
  - Bottom message line

## Festival UI Rules
- Spring Day 28 default bottom line:
  `Today is Butterfly Festival, enjoy it!`
- Guest festival greet override line:
  `✨ Happy Butterfly Festival!`

## Rendering Constraints
- Map dimensions and icons follow active location map definition.
- Non-active entity markers are hidden from unrelated map contexts.
- Wonder/Fountain/river/boundary visuals use tile emoji mapping from world tile types.
