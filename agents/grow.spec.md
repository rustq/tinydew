# Crop Growth Spec

## Status
Implemented.

## Rules
- Growth advances during day transition.
- If crop was watered, `days_grown` increments.
- `watered_today` resets at new day start.
- Crop maturity depends on crop type thresholds.
- Soil cleanup and daily spawn flows run in same day-start cycle.
