# Shop / Trade Spec

## Status
Implemented.

## Behavior
- Trade action opens buy/sell menu flow in interactive mode.
- MCP supports direct `buy:<item>[:qty]` and `sell:<item>[:qty]` commands.
- Money is debited/credited based on item prices and quantities.
- Invalid item/qty or insufficient resources are rejected.
