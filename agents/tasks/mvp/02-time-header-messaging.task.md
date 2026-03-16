# MVP Task 02: Time, Header, and Message Rail

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase B)

## Goal
Add action-driven time progression, weather/night header behavior, and bottom action messages.

## Todo
- [x] Advance time +5 minutes per player action.
- [x] Keep active day flow and day rollover behavior.
- [x] Display header with season/day/weather/time.
- [x] Implement night icon override (🌙 at night regardless of weather).
- [x] Add message rail for action feedback (e.g., "Clear Done!").

## Acceptance
- [x] Time changes only via actions as specified.
- [x] Header values are always consistent with game state.
- [x] Message rail updates after each valid/invalid action.

## Implementation Notes
- Implemented in commit: `2350a92`.
- Added action handlers and ensured each valid action advances in-game clock by 5 minutes.
- Verified night icon override in live run (`☀️` switched to `🌙` at night time window).
- Verified message rail for both success and failure cases (e.g., `Clear Done!`, `Not ready yet!`, `Cannot move there!`, `Trade menu coming soon!`).
