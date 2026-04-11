# Crop Growth Spec

## Status
Implemented.

## Rules
- Growth advances during day transition.
- If crop was watered, `days_grown` increments.
- `watered_today` resets after growth check (check watered → grow → reset).
- All crops mature in 1 day if watered.
- Soil cleanup and daily spawn flows run in same day-start cycle.

## Known Issues
- **All crops mature in 1 day if watered.** Carrot, Strawberry, and Cauliflower all share the same 1-day growth duration. There is no per-crop growth time — any crop planted and watered will be harvestable the very next day. Different crop types should ideally have different days-to-mature values to add strategic depth.
