# Water Action Spec

## Status
Implemented.

## Rules
- Water applies to crop tiles (immature or mature planted crops only).
- Supports adjacent/default targeting.
- Sets crop watered state for growth progression.
- Rainy weather always auto-waters all crops during day transition.
- Watering non-crop tiles (grass, soil, spawned flowers, mushrooms) is a no-op.

## Messaging
- Invalid/non-crop target returns failure message.
