# MVP Task 02: Time, Header, and Message Rail

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase B)

## Goal
Add action-driven time progression, weather/night header behavior, and bottom action messages.

## Todo
- [ ] Advance time +5 minutes per player action.
- [ ] Keep active day flow and day rollover behavior.
- [ ] Display header with season/day/weather/time.
- [ ] Implement night icon override (🌙 at night regardless of weather).
- [ ] Add message rail for action feedback (e.g., "Clear Done!").

## Acceptance
- [ ] Time changes only via actions as specified.
- [ ] Header values are always consistent with game state.
- [ ] Message rail updates after each valid/invalid action.
