# Task: Fishing & Cat System

**Source Spec:** `agents/shelldew.spec.md`
**Source Plan:** `agents/plan/20260314/shelldew-setup.plan.md`
**Workstream:** W4 & W5 - Fishing, Cat, Social
**Milestone:** M4 - Fishing + Cat

## Description

Implement fishing mechanics and cat interaction system. Fishing provides alternative gameplay loop, while cat adds cozy flavor and emotional connection.

## Tasks

### 4.1 Fishing Location Detection
- Implement water-adjacent detection:
  - Check if player is next to water tile (🌊)
  - Support diagonal adjacency
- Add fishing hotspot validation
- Show fishing prompt when near water with rod equipped

### 4.2 Fishing Command Flow
- Implement cast action with `FishingRod` tool:
  - Validate player is near water
  - Validate rod is equipped
  - Start fishing animation/state
- Add short delay (2-3 seconds) for tension
- Determine catch result

### 4.3 Fish Types & Rarity
- Define `Fish` enum with variants:
  - `RiverFish`, `BrightTrout`, `Salmon`, `Catfish`, `RareGoldenFish`
- Add fish metadata:
  - Rarity tiers (common, uncommon, rare, legendary)
  - Base value and sell price
  - Seasonal availability

### 4.4 Fishing Catch Tables
- Create catch tables by location:
  - River: standard freshwater fish
- Add modifiers:
  - Season (summer: more active, winter: less)
  - Weather (rainy: better rates, stormy: rare fish)
  - Time of day (early morning/late evening: better rates)
- Implement weighted random selection

### 4.5 Fishing Result System
- Implement catch determination logic:
  - Roll for catch success
  - Select fish from appropriate table
  - Handle "nothing bit" result
- Add result messages with `emoji`:
  - `🎣 You caught a 🐟 River Fish!`
  - `🎣 You caught a 🐠 Bright Trout!`
  - `🎣 Nothing bit.`
- Add caught fish to inventory

### 4.6 Fishing Tool Integration
- Add `FishingRod` to tool system
- Implement rod equipment logic
- Add rod durability (optional for MVP)
- Show rod in tool slots

### 4.7 Cat Entity System
- Define `Cat` struct with:
  - Position (x, y)
  - Current mood/state
  - Last petted timestamp
  - Daily interaction limit (1 per day)
- Place cat in initial farm location (near house)
- Add cat to world entities

### 4.8 Cat Movement System
- Implement simple cat AI:
  - Move between small set of valid tiles
  - Prefer tiles near house or player
  - Avoid water and obstacles
- Add random movement intervals
- Update cat position on world ticks

### 4.9 Cat Interaction System
- Implement pet interaction:
  - Validate player is adjacent to cat
  - Check daily interaction limit
  - Update last petted timestamp
- Add interaction cooldown (once per day)

### 4.10 Cat Reactions & Dialogue
- Define cat reaction pool:
  - `YouYou pet the cat. 😺`
  - `The cat purrs. 🐾`
  - `The cat stretches in the rain.`
  - `The cat watches the river.`
- Add context-aware reactions:
  - Weather-based (rain, snow)
  - Time-based (sleeping at night)
  - Location-based (near house, near river)
- Display reaction message on interaction

### 4.11 Cat Visuals
- Render cat with emoji (🐱 or 🐈)
- Add cat to map rendering
- Support cat position updates
- Add subtle animations (optional for MVP)

## Acceptance Criteria

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes without warnings
- [ ] Player can fish when adjacent to water with rod equipped
- [ ] Fishing has short delay before result
- [ ] Catch rates vary by season, weather, and time
- [ ] Fish appear in inventory when caught
- [ ] Fishing messages use emoji and are readable
- [ ] Cat appears on farm from early game
- [ ] Player can pet cat once per day
- [ ] Cat reactions are flavorful and context-aware
- [ ] Cat moves among valid tiles
- [ ] Cat rendering works with map system

## Dependencies

- Task 01: Engine & World Loop (entities, rendering)
- Task 03: Environment & Atmosphere (weather, seasons)

## Estimated Effort

- 4-6 hours

## Notes

- Per spec: fishing should be simple cast/wait/result loop
- Cat should feel alive but not complex
- Reactions should be cozy and varied
- Consider adding cat affection score for future polish
- Fishing should feel rewarding but not overpowered
