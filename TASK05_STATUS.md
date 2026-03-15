# Task 05 Status Report: Town & Social System

**Audit Date:** 2026-03-15  
**Task:** agents/tasks/shelldew-setup/05-town-social.task.md  
**Verdict:** COMPLETE ✅

---

## Criterion-by-Criterion Evaluation

| # | Acceptance Criterion | Status | Evidence |
|---|---------------------|--------|----------|
| 1 | `cargo build` succeeds | ✅ PASS | Build completes without errors |
| 2 | `cargo clippy` passes without warnings | ✅ PASS | Zero warnings after fixes |
| 3 | Player can travel between farm and town | ✅ PASS | T key triggers `travel()` in engine/mod.rs:238-256 |
| 4 | Player can enter and exit buildings | ✅ PASS | E key triggers `interact()` at line 200 which calls `enter_building()` for door tiles |
| 5 | NPCs appear in appropriate locations | ✅ PASS | `render_npcs()` in rendering/mod.rs:181 checks location and renders NPCs |
| 6 | NPCs move according to daily schedules | ✅ PASS | `Npc::schedule` HashMap maps TimePhase to Location; `get_location()` returns correct position |
| 7 | Player can talk to NPCs with E key | ✅ PASS | `talk_to_npc()` at town/mod.rs:504 triggered via `interact()` when adjacent to NPC |
| 8 | Dialogue varies by season, weather, and time | ✅ PASS | `Dialogue::select()` at town/mod.rs:159 checks season, weather, time_phase fields |
| 9 | Dialogue is short, readable, and flavorful | ✅ PASS | Sample dialogues in NpcDialogues are concise with emojis (e.g., "🎉 Welcome to my store!") |
| 10 | Shops have inventory with items and prices | ✅ PASS | ShopManager at town/mod.rs:296 defines shops with Vec<ShopItem> including buy_price, sell_price, stock |
| 11 | Player can buy items from shops | ✅ PASS | B key triggers `open_shop(true)` at engine/mod.rs:277-299 |
| 12 | Player can sell items to shops | ✅ PASS | V key triggers `open_shop(false)` for selling mode |
| 13 | Money system works correctly | ✅ PASS | Currency struct with add(), subtract(), amount(); displayed in status bar |
| 14 | Shops close at appropriate times | ✅ PASS | Shop::is_open() checks hour against open_hours (GeneralStore 9-18, Saloon 12-22) |
| 15 | Shop hours affect NPC availability | ✅ PASS | `render_npcs()` checks shop hours before rendering shopkeeper/bartender in their respective shops |

---

## Summary

**All acceptance criteria met.** The task is complete.

### Implementation Details

- **Input Wiring**: All player interactions (T for travel, E for interact/talk, B for buy, V for sell) are wired to the input handler in engine/mod.rs
- **Shop Hours**: NPCs in shops (shopkeeper at GeneralStore, bartender at Saloon) only render when the shop is open
- **UI**: Rendering module displays dialogue bubbles and shop inventory with buy/sell prices
- **Door System**: Added Door, GeneralStoreDoor, SaloonDoor terrain types in world/mod.rs

### Controls

- **WASD**: Move
- **E**: Interact (talk to NPCs, enter buildings)
- **T**: Travel between farm and town
- **B**: Open buy menu (when in shop)
- **V**: Open sell menu (when in shop)
- **Esc**: Close dialogue/shop menu or quit

---

## Notes

- Used V instead of S for sell to avoid conflict with down movement
- Clippy warnings fixed by adding Default implementations, collapsing nested ifs, using is_none_or, matches! macro
