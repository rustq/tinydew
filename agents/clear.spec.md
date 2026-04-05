# Clear Action Spec

## Status
Implemented.

## Rules
- `clear` targets adjacent/default tile according to current action targeting.
- Clearable tiles: grass and plant (immature crop). Clear converts them to soil.
- Soil reverts to grass at the next day transition (after spawn logic, so reverted tiles are not eligible for spawning that same night).
- Cannot clear blocked/building tiles, mature crops, spawned flowers, or mushrooms.
- Only Farm region allows clear actions. Square, EastPath, and SouthRiver forbid clear.

## Messaging
- Invalid/blocked targets return rejection messages (e.g. cannot clear here/target).
