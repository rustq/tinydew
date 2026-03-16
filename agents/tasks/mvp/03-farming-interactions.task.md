# MVP Task 03: Farming Interactions

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase C)

## Goal
Implement C/P/W/H interactions on tile in front of player.

## Todo
- [x] `C` Clear: transform 🌿 -> ▪️ on Farm.
- [x] `P` Plant: transform ▪️ -> 🌱 when seed is available.
- [x] `W` Water: mark crop watered for current day.
- [x] `H` Harvest: mature crop -> 🌿 and add produce to inventory.
- [x] Ensure actions target tile in front (based on facing direction).
- [x] Emit clear success/failure action messages.

## Acceptance
- [x] Full farming loop works clear -> plant -> water -> harvest.
- [x] Invalid actions are safely rejected with user feedback.

## Implementation Notes
- Implemented in commit: `8bef351`.
- Added inventory model for seeds/produce, crop state tracking, and front-tile action logic.
- Added farm-only guardrails with explicit feedback messages for invalid contexts/actions.
- Added action outcomes/messages for clear, plant, water, and harvest paths.
