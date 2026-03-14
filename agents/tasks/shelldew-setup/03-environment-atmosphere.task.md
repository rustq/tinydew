# Task: Environment & Atmosphere

**Source Spec:** `agents/shelldew.spec.md`
**Source Plan:** `agents/plan/20260314/shelldew-setup.plan.md`
**Workstream:** W3 - Environment & Atmosphere
**Milestone:** M3 - Weather & Seasons

## Description

Implement seasonal progression, daily weather system, and atmospheric effects that influence gameplay and visuals. This creates dynamic world that changes over time.

## Tasks

### 3.1 Season System
- Define `Season` enum: `Spring`, `Summer`, `Autumn`, `Winter`
- Implement season cycle (28 days per season)
- Add season transition logic (day 29 → new season, day 1)
- Track current season in `TimeManager`

### 3.2 Seasonal Effects
- Implement season-based terrain palette changes:
  - Spring: fresh grass, blooming trees
  - Summer: vibrant grass, full trees
  - Autumn: orange/brown grass, falling leaves
  - Winter: white ground, bare trees
- Update map rendering based on season
- Add seasonal emoji variants where applicable

### 3.3 Seasonal Crop Availability
- Map crops to valid seasons:
  - Spring: strawberry, carrot, potato
  - Summer: corn, tomato, blueberry
  - Autumn: pumpkin, eggplant, yam
  - Winter: limited/no outdoor crops
- Validate crop planting against current season
- Update shop inventory based on season

### 3.4 Weather System
- Define `Weather` enum: `Sunny`, `Cloudy`, `Rainy`, `Stormy`, `Snowy`
- Implement daily weather roll at sleep/day start
- Add weather probabilities based on season:
  - Spring: balanced mix
  - Summer: mostly sunny, occasional rain
  - Autumn: cloudy, rainy
  - Winter: snowy, cloudy
- Track current weather in world state

### 3.5 Weather Effects on Farming
- Implement rain auto-watering:
  - All outdoor crops marked as watered on rainy days
  - Skip manual watering requirement for that day
- Implement storm effects:
  - May damage crops (optional for MVP)
  - Change fishing catch rates
- Implement snow effects:
  - Disable outdoor crop growth
  - Change ground visuals to frozen

### 3.6 Weather Effects on Fishing
- Define weather-based fishing modifiers:
  - Sunny: normal rates
  - Rainy: increased fish activity
  - Stormy: rare catches but higher value
  - Snowy: reduced or no fishing
- Update fishing catch tables based on weather

### 3.7 Weather Effects on NPCs
- Implement weather-based NPC schedule changes:
  - Rainy: NPCs stay indoors more
  - Stormy: most NPCs shelter
  - Snowy: limited outdoor activity
- Update NPC dialogue references to weather

### 3.8 Time Phase System
- Define `TimePhase` enum: `Morning`, `Day`, `Evening`, `Night`
- Implement phase transitions based on hour:
  - Morning: 05:00-09:00
  - Day: 09:00-17:00
  - Evening: 17:00-21:00
  - Night: 21:00-05:00
- Track current phase in `TimeManager`

### 3.9 Time Phase Effects
- Implement phase-based shop hours:
  - Most shops open during Day phase
  - Some shops open Evening phase
  - Shops close Night phase
- Update NPC locations based on phase
- Add phase-based lighting/visual changes

### 3.10 Status Bar Implementation
- Implement top status bar with:
  - Date display (Season + Day)
  - Time display (HH:MM)
  - Weather emoji (☀️, ☁️, 🌧️, ⛈️, ❄️)
  - Money display (💰 amount)
- Update status bar` on each tick
- Format and align status bar elements

## Acceptance Criteria

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes without warnings
- [ ] Seasons cycle every 28 days
- [ ] Weather rolls daily at day start
- [ ] Seasonal terrain visuals change correctly
- [ ] Crops can only be planted in valid seasons
- [ ] Rain automatically waters all outdoor crops
- [ ] Snow disables outdoor crop growth
- [ ] Fishing catch rates vary by weather
- [ ] Shop hours respect time phases
- [ ] Status bar displays: season, day, time, weather, money
- [ ] Weather transitions smoothly between days

## Dependencies

- Task 01: Engine & World Loop (time system)
- Task 02: Farming System (crops, tiles)

## Estimated Effort

- 5-7 hours

## Notes

- Per spec: no fog weather
- Weather should feel impactful but not punishing
- Season transitions should be noticeable
- Consider adding weather forecast for future polish
- Snow should fundamentally change gameplay pace (focus on indoor activities)
