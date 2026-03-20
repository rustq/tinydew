# Fishing Implementation Tasks

Source:
- `agents/fishing.plan.md`
- `agents/fishing.spec.md`

Objective: implement MCP fishing with river targeting, bubble tile lifecycle, fish inventory/economy integration, persistence, and test coverage.

## Execution Order

1. [01-baseline-and-safety.task.md](./01-baseline-and-safety.task.md)
2. [02-data-model-extensions.task.md](./02-data-model-extensions.task.md)
3. [03-fishing-command-logic-mcp.task.md](./03-fishing-command-logic-mcp.task.md)
4. [04-selling-integration.task.md](./04-selling-integration.task.md)
5. [05-sleep-reset-bubbles.task.md](./05-sleep-reset-bubbles.task.md)
6. [06-mcp-ui-output-integration.task.md](./06-mcp-ui-output-integration.task.md)
7. [07-persistence.task.md](./07-persistence.task.md)
8. [08-tests-and-verification.task.md](./08-tests-and-verification.task.md)
9. [09-manual-verification.task.md](./09-manual-verification.task.md)

## Suggested Commit Sequence

1. `feat(fishing): add fish inventory model and fish types`
2. `feat(tiles): add RiverBubble tile state and rendering`
3. `feat(mcp): add fishing command with time cost and probability outcomes`
4. `feat(economy): support fish selling values and income tracking`
5. `fix(reset): reset RiverBubble tiles on sleep cycle`
6. `test(fishing): add fishing+bubble unit and MCP coverage`
7. `docs(fishing): align spec and implementation notes`
