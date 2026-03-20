# Fishing Task 08: Tests & Verification

Source:
- `agents/fishing.plan.md` (Phase 7)
- `agents/fishing.spec.md` (Suggested Tests + Acceptance Criteria)

## Goal
Add robust coverage for fishing behavior and regressions.

## Todo
- [x] Add tests for fishing availability/validation:
  - [x] `test_fishing_requires_nearby_river`
  - [x] `test_mcp_parse_fishing_command`
- [x] Add tests for time/effects/probabilities:
  - [x] `test_fishing_advances_time_by_one_hour`
  - [x] `test_fishing_outcome_common_fish`
  - [x] `test_fishing_outcome_rare_fish`
  - [x] `test_fishing_outcome_nothing`
  - [x] `test_fishing_adds_items_to_inventory`
- [x] Add tests for messaging and economy:
  - [x] `test_fishing_success_shows_congrats_message`
  - [x] `test_fish_sell_values`
- [x] Add tests for tile lifecycle:
  - [x] `test_river_tile_turns_into_bubble_after_fishing`
  - [x] `test_river_bubble_tile_is_still_fishable`
  - [x] `test_sleep_cycle_resets_bubble_to_river`
- [x] Add persistence tests:
  - [x] `test_fish_inventory_persists_save_load`
  - [x] `test_river_bubble_state_persists_save_load`
- [x] Run full relevant test suite and capture outcomes.

## Acceptance
- [x] New fishing and bubble coverage passes.
- [x] No regressions in existing gameplay/MCP tests.

---

## Completed

- Verified test coverage exists for all planned fishing cases in spec/plan:
  - availability/validation
  - parser support (including directional fishing variant in MCP parser)
  - time advancement
  - outcome branches (common/rare/nothing)
  - inventory mutation
  - messaging
  - fish sell values
  - bubble tile lifecycle
  - persistence roundtrip for fish inventory + bubble state.
- Confirmed test identifiers present:
  - `test_fishing_requires_nearby_river`
  - `test_parse_fishing` (+ `test_parse_fishing_with_direction`)
  - `test_fishing_advances_time_by_one_hour`
  - `test_fishing_outcome_common_fish`
  - `test_fishing_outcome_rare_fish`
  - `test_fishing_outcome_nothing`
  - `test_fishing_adds_items_to_inventory`
  - `test_fishing_success_shows_congrats_message`
  - `test_fish_sell_values`
  - `test_river_tile_turns_into_bubble_after_fishing`
  - `test_river_bubble_tile_is_still_fishable`
  - `test_sleep_cycle_resets_bubble_to_river`
  - `test_fish_inventory_persists_save_load`
  - `test_river_bubble_state_persists_save_load`
- Full relevant suite run result:
  - `cargo test` -> **252 passed, 0 failed, 1 ignored**.
