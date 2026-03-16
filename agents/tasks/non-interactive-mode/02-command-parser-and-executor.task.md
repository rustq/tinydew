# Non-Interactive Task 02: Command Parser & Sequential Executor

Source:
- `agents/non-interactive-mode.spec.md`
- `agents/non-interactive-mode.plan.md` (Phase 2)

## Goal
Implement the command parser and deterministic sequential command execution for non-interactive runs.

## Todo
- [ ] Create non-interactive runner module (e.g., `src/non_interactive.rs`).
- [ ] Parse and dispatch supported commands:
  - [ ] `move:<direction>`
  - [ ] `clear`
  - [ ] `plant:<crop>` (crop required)
  - [ ] `water`
  - [ ] `harvest`
  - [ ] `buy:<item>[:<qty>]`
  - [ ] `sell:<item>[:<qty>]`
  - [ ] `sleep`
  - [ ] `print`
  - [ ] `quit`
- [ ] Reject unsupported `trade` command in batch mode with explicit error.
- [ ] Execute commands strictly in provided order.
- [ ] Stop on first error and return exit code `1`.
- [ ] On full success, return exit code `0`.

## Acceptance
- [ ] Multiple `-c` commands run in exact order.
- [ ] `plant` without crop (e.g., `plant`) is invalid.
- [ ] Invalid direction/crop/item/qty produce clear stderr errors.
- [ ] `quit` terminates remaining command execution and exits success (`0`) when no prior error.

## Notes
- Reuse existing game action/state methods to avoid behavior drift from TUI.
- Validate qty as positive integer (`0` and negatives invalid).