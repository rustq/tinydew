# MVP Task 03: Farming Interactions

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase C)

## Goal
Implement C/P/W/H interactions on tile in front of player.

## Todo
- [ ] `C` Clear: transform 🌿 -> ▪️ on Farm.
- [ ] `P` Plant: transform ▪️ -> 🌱 when seed is available.
- [ ] `W` Water: mark crop watered for current day.
- [ ] `H` Harvest: mature crop -> 🌿 and add produce to inventory.
- [ ] Ensure actions target tile in front (based on facing direction).
- [ ] Emit clear success/failure action messages.

## Acceptance
- [ ] Full farming loop works clear -> plant -> water -> harvest.
- [ ] Invalid actions are safely rejected with user feedback.
