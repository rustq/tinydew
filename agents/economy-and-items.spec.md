# Economy and Items Spec

## Status
Implemented.

## Inventory Model
- Seeds
- Produce
- Forage
- Fish

## Seeds/Crops
- Shop sells one generic seed item (`seed`).
- Seed inventory is tracked as a generic seed count (displayed as 🫙 xN in TUI and `tinydew status` output).
- Planting consumes one seed and randomly rolls a crop type.
- Crops grow via day transitions when watered.
- Mature crops harvest into produce inventory.
- Mushrooms harvest into forage inventory (`🍄`).

## Shop/Trade
- TUI trade menu supports buy/sell flow.
- CLI: `tinydew do buy …` and `tinydew do sell …` (see `cli.spec.md`).
- Validation rejects invalid item/qty or insufficient resources.

## Money
- Buying decreases money by price × qty.
- Selling increases money by value × qty.
- Mushroom sells for $25 each (e.g. `tinydew do sell 🍄`).
- Daily income summary fields remain in state for compatibility, but no forced end-of-day income screen is used.

## Persistence/Output
- Inventory and money persist in the SQLite game database (see `single-state-no-session.spec.md`).
- Both are visible in the TUI and in `tinydew status` output.
