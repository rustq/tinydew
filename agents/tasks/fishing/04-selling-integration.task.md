# Fishing Task 04: Selling Integration

Source:
- `agents/fishing.plan.md` (Phase 3)
- `agents/fishing.spec.md` (Selling integration + Economy Rules)

## Goal
Enable fish sale via existing sell mechanics and accounting.

## Todo
- [x] Extend sell command/menu logic to support fish item categories.
- [x] Apply exact sell values:
  - [x] `🐟` = `$80`
  - [x] `🐠` = `$180`
- [x] Ensure sale updates:
  - [x] player money
  - [x] daily income/accounting structures
- [x] Keep sell validation behavior consistent with existing item sales.

## Acceptance
- [x] Fish can be sold through normal selling flow.
- [x] Money and income values update correctly.

---

## Completed

- MCP sell parser supports fish item categories in `src/mcp/command.rs`:
  - `sell:fish[:qty]` -> common fish (`🐟`)
  - `sell:rare[:qty]` / `sell:tropical[:qty]` -> rare fish (`🐠`)
- Fish inventory sale flow is implemented:
  - `Inventory::sell_fish(...)` decrements fish stock when available.
- Exact sell values are implemented in `src/world.rs`:
  - `FishType::Common` -> `$80`
  - `FishType::Rare` -> `$180`
- Selling fish updates both economy and accounting:
  - player `money += revenue`
  - `record_income(revenue)` updates daily money earned
  - `record_fish_sold(fish, count)` updates daily fish sold counts.
- Validation behavior is consistent with existing sell flows:
  - when none available: `No <emoji> to sell!`
  - partial/limited stock selling handled by loop with `sold_count`.
- Verified with targeted tests:
  - `test_parse_sell_fish`
  - `test_execute_sell_fish`
  - `test_fish_sell_values`
  - all passing.
