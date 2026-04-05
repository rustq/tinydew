# Crop Growth Spec

## Status
Implemented.

## Rules
- Growth advances during day transition.
- If crop was watered, `days_grown` increments.
- `watered_today` resets after growth check (check watered → grow → reset).
- All crops mature in 1 day if watered.
- Soil cleanup and daily spawn flows run in same day-start cycle.
