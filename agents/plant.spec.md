# Plant Action Spec

## Status
Implemented.

## Rules
- Plant consumes one generic seed (`seed`).
- CLI planting: `tinydew do plant <dir>` (consumes one seed; see `cli.spec.md`).
- Planted result randomly rolls to one crop type (Carrot/Strawberry/Cauliflower/Flower).
- Plant can target an adjacent tile per directional/action rules shared with other `do` commands.
- Valid planting requires plantable ground and region allowance.
- Square and EastPath are non-farm planting zones (blocked).

## Messaging
- Invalid tile/region/seed shortage returns failure message.
