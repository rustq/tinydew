# Harvest Action Spec

## Status
Implemented.

## Rules
- Harvest collects produce from mature crops.
- Supports adjacent/default targeting.
- Mushroom and flower after harvest return to soil.
- Crop (carrot, strawberry, cauliflower) after harvest returns to plant (immature crop) state, keeping the same crop type. `days_grown` resets to 0 and the replanted crop is not watered; it must be watered again to mature.
- Harvested mushroom and flower are added to forage inventory.
- Harvested crops are added to produce inventory.
- Produce is added into inventory counters.
- Only Farm region allows harvest of planted crops. Spawned flowers/mushrooms can be harvested in any region.
