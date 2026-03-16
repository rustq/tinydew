# MVP Task 07: Sleep, Income, and Day Loop

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase G)

## Goal
Implement end-of-day sleep flow and income summary screen.

## Todo
- [x] Show Home alert behavior according to spec.
- [x] Enforce mandatory sleep flow at 02:00.
- [x] On sleep: advance day, roll weather, reset daily state.
- [x] Display "Income this day" summary screen with totals/items.
- [x] Return to next day gameplay cleanly.

## Acceptance
- [x] Day transition is stable and repeatable.
- [x] Income summary is shown each sleep cycle.

## Completed
- Implemented `HomeState` enum (None, Alert, Income) in state.rs
- Added `DailyIncome` struct to track money and items sold each day
- Added `check_home_alert()` - triggers at 02:00 when on Farm
- Added `record_income()` and `record_crop_sold()` to track earnings
- Added rendering for Home alert and Income summary screens
- Added input handling for home menu (Enter to confirm)
- Added tests: home_alert_triggers_at_2am, home_alert_not_in_east_path, sleep_transitions_to_income, income_tracks_earnings, close_home_resets_income
- All verification commands pass: cargo build, cargo test, cargo clippy, cargo fmt --check
