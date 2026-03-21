# Economy and Items Spec

## Status
Implemented.

## Inventory Model
- Seeds
- Produce
- Fish

## Seeds/Crops
- Shop sells one generic seed item (`seed`).
- Seed inventory is tracked as a generic seed count (displayed as 🫙 xN in MCP snapshot UI).
- Planting consumes one seed and randomly rolls a crop type.
- Crops grow via day transitions when watered.
- Mature crops harvest into produce inventory.
- Mushrooms are treated as mature crop pickups (not a separate forage inventory type).

## Shop/Trade
- Interactive trade menu supports buy/sell flow.
- MCP supports `buy:<item>[:qty]` and `sell:<item>[:qty]`.
- Validation rejects invalid item/qty or insufficient resources.

## Money
- Buying decreases money by price × qty.
- Selling increases money by value × qty.
- Mushroom-equivalent pickups are represented under crop/produce flow (no separate forage sell path in spec).
- Daily income summary fields remain in state for compatibility, but no forced end-of-day income screen is used.

## Persistence/Output
- Inventory and money persist via save/load.
- Both are exposed in MCP state/stats/snapshot output.
