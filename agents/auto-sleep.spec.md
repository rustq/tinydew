# Auto Sleep System Specification

## Overview
Define automatic sleep behavior when the in-game clock reaches **02:00**.

At 02:00, the player should:
1. Automatically fall asleep
2. Enter the normal sleep resolution flow (end-of-day loop)
3. See income summary
4. Wake at the **front of home** on the next day

---

## 1) Goals

1. Enforce a hard daily cutoff to prevent infinite overnight play.
2. Reuse existing sleep/day-transition logic (single source of truth).
3. Keep behavior deterministic and easy to test.

---

## 2) Non-Goals

1. No stamina penalty tuning in this spec.
2. No cinematic/animation requirements in this spec.
3. No random wake location behavior.

---

## 3) Trigger Rule

## 3.1 Auto Sleep Trigger

Auto sleep triggers when game time becomes exactly **02:00**.

- Trigger condition: `hour == 2 && minute == 0`
- Trigger must fire **once per day**
- If time jumps over boundaries (e.g., command batch), trigger on first state evaluation at/after 02:00 before any further player actions

## 3.2 Priority

At trigger time, auto sleep has higher priority than normal free-roam input.

---

## 4) Behavior Flow

When auto sleep is triggered:

1. Lock gameplay input for free-roam commands.
2. Execute the same sleep pipeline as manual sleep:
   - finalize day
   - process overnight updates
   - compute daily income summary
3. Show income summary screen/message.
4. Advance to next day at wake time.
5. Spawn player at **front of home**.
6. Return control to normal gameplay.

---

## 5) Time & Day Transition

## 5.1 Day Rollover

- Current day ends at auto-sleep resolution.
- Next day starts after sleep loop completion.

## 5.2 Wake Time

Use existing wake-time convention already used by manual sleep (do not fork a new rule in this spec).

---

## 6) Spawn Rule After Auto Sleep

Wake position must match the existing canonical “front of home” spawn.

- No random offset
- No carry-over of previous location
- Same spawn as manual sleep wake behavior

---

## 7) Income Summary Requirements

Income summary displayed in auto sleep must match manual sleep summary semantics:

1. Show total income for the day
2. Show relevant breakdown (if currently supported)
3. Update player money before returning control

If income is zero, still show a valid summary state (e.g., total 0).

---

## 8) Save/Load Behavior

1. If saved before 02:00, auto sleep should still trigger when time reaches 02:00 after load.
2. If saved during sleep-resolution state, loading should resume safely in a consistent post-sleep state or re-enter sleep resolution deterministically (implementation choice must be consistent).
3. No data migration required if existing day/sleep fields already support manual sleep.

---

## 9) MCP / Command Behavior

1. MCP command execution at/after 02:00 should resolve auto sleep before allowing additional world actions.
2. Batch commands that cross into 02:00 should stop normal action processing, run auto sleep, then continue only under defined post-sleep rules (recommended: stop batch and require next turn).
3. Response events should include explicit sleep/day-transition markers so clients can react.

---

## 10) Edge Cases

1. **Exact boundary:** 01:59 -> 02:00 triggers immediately.
2. **Large time jump:** If system jumps from before 02:00 to after 02:00, auto sleep still triggers once.
3. **Already sleeping:** Prevent duplicate trigger while sleep flow is active.
4. **Menu/pause at 02:00:** On resume/state tick, auto sleep triggers before normal input.
5. **Multiple triggers:** Ensure one trigger per day via guard flag/state transition.

---

## 11) Implementation Notes

## 11.1 Suggested Helpers

- `fn should_auto_sleep(hour: u8, minute: u8, state: &GameState) -> bool`
- `fn run_auto_sleep(state: &mut GameState) -> SleepResult`

## 11.2 State Guard

Add/derive guard to avoid repeated triggers during same night window:

- e.g. `auto_sleep_triggered_for_day: u32` or equivalent derived transition state.

## 11.3 Reuse Existing Pipeline

Auto sleep must call the same underlying function/path used by manual sleep to avoid divergence.

---

## 12) Testing Plan

## 12.1 Unit Tests

1. At `02:00`, `should_auto_sleep == true`.
2. At `01:59`, `should_auto_sleep == false`.
3. At `02:01`, auto sleep still resolves once if crossing logic supports catch-up.
4. Trigger does not repeat for same day after completion.

## 12.2 Integration Tests

1. Simulate time progression to 02:00 -> verify sleep loop runs.
2. Verify income summary appears.
3. Verify day increments.
4. Verify wake location is front of home.
5. Verify control returns to normal next-day gameplay.

## 12.3 MCP Tests

1. Command at 01:59 then next tick to 02:00 -> auto sleep event appears.
2. Batch crossing 02:00 -> expected stop/transition behavior is enforced.

---

## 13) Rollout Plan

1. Add trigger evaluation at central time-advance point.
2. Hook trigger to existing sleep resolver.
3. Add explicit transition/income events.
4. Add tests (unit + integration + MCP).
5. Validate no regression for manual sleep.

---

## 14) Acceptance Criteria

Feature is complete when:

1. Player auto-falls asleep at **02:00**.
2. Auto sleep uses the normal sleep game loop.
3. Income summary is shown during the sleep resolution.
4. Player wakes at the **front of home**.
5. Behavior is deterministic and triggers only once per day.
6. Manual sleep behavior remains unchanged.
