# Task: Town & Social System

**Source Spec:** `agents/shelldew.spec.md`
**Source Plan:** `agents/plan/20260314/shelldew-setup.plan.md`
**Workstream:** W4 - Social, Town, Cat, and Commerce
**Milestone:** M5 - Town & Social

## Description

Implement town locations, NPC system, dialogue, and social interactions. Town provides shopping, social interaction, and sense of community.

## Tasks

### 5.1 Town Map Structure
- Create town map data (separate from farm map)
- Define town areas:
  - Town square
  - General store
  - Saloon
  - Town hall
- Add town transition points (farm ↔ town)
- Implement map switching system

### 5.2 Shop Interior System
- Create shop interior maps:
  - General store interior
  - Saloon interior
- Add shop-specific NPCs (shopkeepers)
- Implement enter/exit logic for buildings
- Add interior navigation

### 5.3 NPC Definition System
- Define `NPC` struct with:
  - Name
  - Map locations (farm, town, interiors)
  - Daily schedule (time → location)
  - Dialogue pools
  - Friendship/familiarity value
- Create initial NPC roster:
  - Shopkeeper
  - Saloon bartender
  - 2-3 villagers
- Add NPC to world entities

### 5.4 NPC Schedule System
- Implement time-based NPC positioning:
  - Check current time phase
  - Move NPC to scheduled location
  - Support multiple locations per day
- Update NPC positions on time ticks
- Add schedule variations by season/weather

### 5.5 Dialogue System
- Define `Dialogue` struct with:
  - Text content
  - Conditions (season, weather, time, relationship)
  - Response options (optional)
- Create dialogue templates keyed by:
  - NPC ID
  - Season
  - Weather
  - Time phase
  - Relationship level
- Implement dialogue selection logic

### 5.6 Dialogue Factors
- Implement season-based dialogue:
  - Spring: hopeful, planting topics
  - Summer: energetic, outdoor activities
  - Autumn: harvest, cozy topics
  - Winter: indoor, reflection
- Implement weather-based dialogue:
  - Rainy: staying dry, cozy
  - Sunny: energetic, plans
  - Snowy: cold, winter activities
- Implement time-based dialogue:
  - Morning: greetings, plans
  - Day: activities, work
  - Evening: winding down
  - Night: tired, sleeping

### 5.7 NPC Interaction System
- Implement talk interaction (E key):
  - Validate player is adjacent to NPC
  - Select appropriate dialogue
  - Display dialogue with emoji
- Add dialogue history/log (T key)
- Support dialogue options (future polish)

### 5.8 Shop Inventory System
- Define `ShopInventory` struct with:
  - Available items
  - Stock quantities
  - Buy prices
  - Sell prices
- Create shop-specific inventories:
  - General store: seeds, tools, supplies
  - Saloon: food, drinks
- Update stock based on season

### 5.9 Shopping System
- Implement browse functionality:
  - Display shop inventory with prices
  - Show item descriptions
- Implement buy functionality:
  - Validate player has enough money
  - Validate shop has stock
  - Deduct money, add item to inventory
  - Reduce shop stock
- Add shopping UI

### 5.10 Selling System
- Implement sell functionality:
  - Display player inventory with sell prices
  - Validate item is sellable
  - Add money to player
  - Remove item from inventory
- Add selling UI
- Implement price display (buy vs sell)

### 5.11 Currency System
- Define `Currency` struct tracking player money
- Implement money operations:
  - Add (earn from selling)
  - Subtract (spend on buying)
  - Validate sufficient funds
- Add money display in status bar
- Implement money persistence

### 5.12 Shop Hours System
- Implement shop open/close logic:
  - General store: open Day phase
  - Saloon: open Day + Evening phases
  - Close Night phase
- Add door lock interaction when closed
- Display shop status in UI

### 5.13 Town Navigation
- Implement map transitions:
  - Farm → Town (edge of farm)
  - Town → Farm (edge of town)
  - Enter buildings (E on door)
  - Exit buildings (E on door)
- Add transition animations (optional for MVP)
- Preserve player state across maps

## Acceptance Criteria

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes without warnings
- [ ] Player can travel between farm and town
- [ ] Player can enter and exit buildings
- [ ] NPCs appear in appropriate locations
- [ ] NPCs move according to daily schedules
- [ ] Player can talk to NPCs with E key
- [ ] Dialogue varies by season, weather, and time
- [ ] Dialogue is short, readable, and flavorful
- [ ] Shops have inventory with items and prices
- [ ] Player can buy items from shops
- [ ] Player can sell items to shops
- [ ] Money system works correctly
- [ ] Shops close at appropriate times
- [ ] Shop hours affect NPC availability

## Dependencies

- Task 01: Engine & World Loop (maps, entities)
- Task 02: Farming System (inventory, tools)
- Task 03: Environment & Atmosphere (time, seasons, weather)

## Estimated Effort

- 7-9 hours

## Notes

- Per spec: town should feel useful and alive
- Dialogue should be cozy, not verbose
- Shop prices should be balanced (sell < buy)
- NPC schedules should feel natural
- Consider adding relationship progression for future polish
- Town map should be small but explorable
