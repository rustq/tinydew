# Plant Action Spec

## Status
Implemented.

## Rules
- Plant consumes one generic seed (`seed`).
- CLI planting: `tinydew do plant <dir>` (consumes one seed; see `cli.spec.md`).
- Planted result randomly rolls to one crop type (Carrot/Strawberry/Cauliflower).
- Plant can target an adjacent tile per directional/action rules shared with other `do` commands.
- Valid planting requires soil tile and region allowance.
- Only Farm region allows planting. Square, EastPath, and SouthRiver forbid planting.

## Messaging
- Invalid tile/region/seed shortage returns failure message.
