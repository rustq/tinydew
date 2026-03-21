# Plant Action Spec

## Status
Implemented.

## Rules
- Plant consumes one generic seed (`seed`).
- MCP planting command is `plant:seed` (optionally with direction).
- Planted result randomly rolls to one crop type (Carrot/Strawberry/Cauliflower/Flower).
- Plant can target adjacent/default tile according to MCP action grammar.
- Valid planting requires plantable ground and region allowance.
- Square and EastPath are non-farm planting zones (blocked).

## Messaging
- Invalid tile/region/seed shortage returns failure message.
