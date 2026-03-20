# Plant Action Spec

## Status
Implemented.

## Rules
- Plant consumes seed inventory for selected crop.
- Plant can target adjacent/default tile according to MCP action grammar.
- Valid planting requires plantable ground and region allowance.
- Square and EastPath are non-farm planting zones (blocked).

## Messaging
- Invalid tile/region/seed shortage returns failure message.
