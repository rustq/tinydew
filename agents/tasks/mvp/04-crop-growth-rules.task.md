# MVP Task 04: Crop Growth Rules

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Phase D)

## Goal
Implement Spring crop catalog and growth lifecycle with watering dependency.

## Todo
- [ ] Add crop definitions:
  - [ ] Carrot (4 days)
  - [ ] Strawberry (8 days)
  - [ ] Cauliflower (12 days)
  - [ ] Rhubarb (16 days)
- [ ] Implement growth progression logic per day.
- [ ] Enforce “not watered => no growth”.
- [ ] Render 🌱 while growing; render mature emoji at completion.
- [ ] Reset daily watered flag correctly at day transition.

## Acceptance
- [ ] All four crops mature at correct day counts.
- [ ] Watering requirement is strictly enforced.
