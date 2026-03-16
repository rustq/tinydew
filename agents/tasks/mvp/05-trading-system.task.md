# MVP Task 05: Trading System

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase E)

## Goal
Implement `T` shop flow for buying seeds and selling crops.

## Todo
- [x] Open shop menu with `T`.
- [x] Implement menu navigation (↑ ↓ Enter).
- [x] Add buy entries for four seed jars in spec order.
- [x] Deduct money and add seed inventory on buy.
- [x] Implement sell menu showing owned crops only.
- [x] Add money and deduct crop count on sell.
- [x] Add Exit/Back flow.

## Acceptance
- [x] Shop interaction is fully keyboard-operable.
- [x] Buy/sell transactions update inventory and money correctly.

## Completed
- Added `ShopState` enum and shop state tracking in GameState
- Buy menu: Carrot seed $10, Strawberry $20, Cauliflower $30, Rhubarb $40
- Sell menu: Shows only crops player has (Carrot $20, Strawberry $40, Cauliflower $60, Rhubarb $80)
- Keyboard navigation with ↑↓ and Enter
- Esc to close/back from shop
- Money starts at $500

## Validation Snapshot
- Verified Sell menu contains harvested carrot entry: `Sell 🥕 Carrot ($20)`.
- Confirmed transaction updates wallet from `$500` to `$520` after selling one carrot.
- Confirmed produce depletion behavior: Sell list changes to `(No crops to sell)` after sale.
