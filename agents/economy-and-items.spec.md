# Economy and Items Spec

## Status
Implemented.

## Inventory Model
- Seeds
- Produce
- Forage
- Fish

## Seeds/Crops
- Seeds are bought and tracked by crop type.
- Planting consumes seeds.
- Crops grow via day transitions when watered.
- Mature crops harvest into produce inventory.

## Shop/Trade
- Interactive trade menu supports buy/sell flow.
- MCP supports `buy:<item>[:qty]` and `sell:<item>[:qty]`.
- Validation rejects invalid item/qty or insufficient resources.

## Money
- Buying decreases money by price × qty.
- Selling increases money by value × qty.
- Daily income summary fields are tracked for day reporting.

## Persistence/Output
- Inventory and money persist via save/load.
- Both are exposed in MCP state/stats/snapshot output.
