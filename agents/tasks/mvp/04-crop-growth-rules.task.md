# MVP Task 04: Crop Growth Rules

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase D)

## Goal
Implement Spring crop catalog and growth lifecycle with watering dependency.

## Todo
- [x] Add crop definitions:
  - [x] Carrot (4 days)
  - [x] Strawberry (8 days)
  - [x] Cauliflower (12 days)
  - [x] Rhubarb (16 days)
- [x] Implement growth progression logic per day.
- [x] Enforce "not watered => no growth".
- [x] Render 🌱 while growing; render mature emoji at completion.
- [x] Reset daily watered flag correctly at day transition.

## Acceptance
- [x] All four crops mature at correct day counts.
- [x] Watering requirement is strictly enforced.

## Completed
- Implemented `start_new_day()` method in state.rs that processes crop growth
- Crops only advance `days_grown` when `watered_today` is true
- `watered_today` is reset to false at the start of each new day
- Growth days: Carrot=4, Strawberry=8, Cauliflower=12, Rhubarb=16
- Rendering: 🌱 for growing, crop emoji for mature
