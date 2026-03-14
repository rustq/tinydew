# Task: Farming System

**Source Spec:** `agents/shelldew.spec.md`
**Source Plan:** `agents/plan/20260314/shelldew-setup.plan.md`
**Workstream:** W2 - Farming Systems
**Milestone:** M2 - Farming Loop

## Description

Implement core farming mechanics including terrain manipulation, crop lifecycle, tool interactions, and inventory integration. This enables the core gameplay loop of clearing, tilling, planting, watering, and harvesting.

## Tasks

### 2.1 Terrain & Tile State System
- Extend `Tile` enum with farming states:
  - `Tilled`, `Planted { crop_type, stage, watered }`, `Harvested`
- Add crop growth stages: `Seed`, `Sprout`, `Growing`, `Mature`
- Implement tile state transitions (grass → tilled → planted → mature)
- Add tile metadata (last watered time, growth progress)

### 2.2 Tool System
- Define `Tool` enum: `Hoe`, `WateringCan`, `Axe`, `Pickaxe`, `FishingRod`
- Create tool slot system (1-9 for quick access)
- Implement tool selection and equipment UI
- Add tool-specific interaction logic

### 2.3 Clearing Mechanics
- Implement `Axe` interaction with wood tiles (🪵)
- Implement `Pickaxe` interaction with stone tiles (🪨)
- Implement `Hoe` interaction with weed tiles (🌾)
- Add drop system (wood, stone, seeds from clearing)
- Update tile states after clearing

### 2.4 Tilling System
- Implement `Hoe` interaction with grass tiles (🌿)
- Convert grass → tilled soil (🌱)
- Validate tilling conditions (must be grass, not already tilled)
- Add visual feedback for tilling action

### 2.5 Crop Types & Seasons
- Define `CropType` enum: `Strawberry`, `Corn`, `Tomato`, `Pumpkin`, `Carrot`, `Eggplant`, `Blueberry`
- Add crop metadata:
  - Season availability (spring, summer, autumn, winter)
  - Growth duration (in days)
  - Watering requirements
  - Harvest yield
- Implement season-based crop validation

### 2.6 Planting System
- Implement seed usage on tilled soil
- Validate planting conditions:
  - Tile must be tilled
  - Crop must be in season
  - Player must have seeds in inventory
- Create planted tile state with initial crop data
- Deduct seeds from inventory

### 2.7 Watering System
- Implement `WateringCan` interaction with planted tiles
- Update tile watered state and timestamp
- Enforce 10-minute cooldown per tile (per spec)
- Add visual indicator for watered tiles
- Support rain auto-watering (weather integration point)

### 2.8 Crop Growth System
- Implement daily growth tick
- Check growth conditions:
  - Tile is planted
  - Crop is in season
  - Tile was watered today (or it rained)
- Advance crop stage based on growth duration
- Prevent growth in winter (per spec)

### 2.9 Harvesting System
- Implement harvest interaction with mature crops
- Validate harvest conditions:
  - Crop is mature stage
  - Player has inventory space
- Add harvested crops to inventory
- Reset tile to tilled state
- Show harvest result message with emoji

### 2.10 Inventory System
- Create `Inventory` struct with item slots
- Implement item stack system (quantity per item type)
- Add inventory UI (I key to view)
- Support add/remove operations
- Implement capacity limits

## Acceptance Criteria

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes without warnings
- [ ] Player can clear weeds, wood, and stones with appropriate tools
- [ ] Player can till grass into soil with hoe
- [ ] Player can plant seeds on tilled soil
- [ ] Player can water planted crops
- [ ] Watering has 10-minute cooldown per tile
- [ ] Crops grow daily when they are:
  - In season
  - Watered (or rained on)
- [ ] Player can harvest mature crops
- [ ] Harvested crops appear in inventory
- [ ] Inventory UI displays items correctly
- [ ] Tools can be selected via 1-9 keys
- [ ] Winter prevents outdoor crop growth

## Dependencies

- Task 01: Engine & World Loop (must have map, tiles, input, time systems)

## Estimated Effort

- 6-8 hours

## Notes

- Per spec: no stamina system, so tool use is unlimited
- Growth timing should feel rewarding but not too fast
- Consider adding crop failure conditions for future polish
- Inventory should be simple for MVP (no sorting/filtering initially)
