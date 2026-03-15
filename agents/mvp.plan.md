# Shelldew MVP Coding Plan

Source: `agents/mvp.spec.md`

## 1) Objective

Build a minimal CLI farming game that matches the MVP spec:

- Emoji tile world
- Two playable areas (Farm + East Path)
- Action-driven time progression
- Farming loop (clear, plant, water, harvest)
- Trading loop (buy seeds, sell crops)
- Day-end sleep + income summary

---

## 2) Scope Baseline

### In Scope (MVP)

- 8×8 Farm map
- East Path dead-end forage map
- 4 spring crops (Carrot, Strawberry, Cauliflower, Rhubarb)
- Crop growth timers + watering requirement
- Weather/day/night header behavior
- Trading menu with buy/sell
- Mushroom forage spawn in East Path
- Back home sleep flow + daily income summary

### Out of Scope (MVP)

- Animals
- Crafting
- Energy/stamina
n- NPC town systems
- Automation systems

---

## 3) Architecture Plan

## 3.1 Core Modules

- `world`:
  - map data structures
  - tile types and transitions
  - area switching (Farm ↔ East Path)
- `engine`:
  - game loop
  - input dispatch
  - action execution and turn/time advancement
- `time`:
  - clock state (06:00–22:00 active period)
  - day progression
  - weather rolls per day
  - night icon override
- `farming`:
  - crop definitions and growth stages
  - watering flags and daily growth checks
- `inventory`:
  - seed stock
  - harvested crops
  - forage items
- `trading`:
  - shop UI state
  - buy/sell handlers
  - pricing and money updates
- `ui/rendering`:
  - header + map + message area
  - alert/dialog modes (sleep/income/shop)

## 3.2 Data Models (MVP)

- `TileType`: Tree, Grass, Soil, Crop(crop_id, stage), House, PathEast, PathFarm, Mushroom
- `CropType`: Carrot, Strawberry, Cauliflower, Rhubarb
- `CropState`: planted_day, watered_today, days_grown, mature
- `GameState`:
  - location
  - map states
  - player position/direction
  - day/time/season/weather
  - wallet
  - inventory
  - action message

---

## 4) Implementation Phases

## Phase A — World + Movement Foundation

### A1. Farm map + tile schema

- Implement exact 8×8 farm boundary and house layout from spec.
- Add passability rules:
  - blocked: 🌳
  - walkable: 🌿 ▪️ 🌱 mature crops

### A2. East Path map

- Implement fixed dead-end map with transition back tile (⬅️).
- Disable farming actions in East Path.

### A3. Movement controls

- Arrow key movement only for MVP mode.
- Keep facing direction for “tile in front” interactions.

Acceptance:
- Player moves correctly and cannot pass boundaries.
- Transitions between Farm and East Path work.

---

## Phase B — Time + Header + Messaging

### B1. Action-based time advancement

- Advance time by +5 min per action.
- Maintain day window (06:00 onward).

### B2. Night icon override

- At night display 🌙 regardless of day weather icon.

### B3. Message rail

- Bottom UI message like `Clear Done!`, `Plant Done!`, errors, etc.

Acceptance:
- Header always shows season/day/weather/time correctly.
- Action messages display after each action.

---

## Phase C — Farming Loop

### C1. Clear action (`C`)

- Target tile in front.
- Transform `🌿 -> ▪️` on Farm only.

### C2. Plant action (`P`)

- Transform `▪️ -> 🌱` if seed exists.
- Deduct one selected/default seed.

### C3. Water action (`W`)

- Mark planted crop as watered for current day.
- Growth only advances when watered.

### C4. Harvest action (`H`)

- Mature crop `🥕🍓🥦🌺 -> 🌿`.
- Add harvested produce to inventory.

Acceptance:
- Full cycle works: clear → plant → water → grow → harvest.

---

## Phase D — Crop Growth Rules

### D1. Crop catalog

- Carrot: 4 days
- Strawberry: 8 days
- Cauliflower: 12 days
- Rhubarb: 16 days

### D2. Growth processor

- On day change, advance watered crops only.
- If not watered, no growth.

### D3. Visual maturity

- Growing: 🌱
- Mature: crop emoji

Acceptance:
- Each crop matures at specified day count.

---

## Phase E — Trading System (`T`)

### E1. Shop menu

- Implement menu entries in spec order.
- Keyboard navigation: ↑ ↓ Enter.

### E2. Buy seeds

- Add seed inventory by type.
- Deduct money.

### E3. Sell crops

- Show only available sellable items.
- Add money, deduct crop counts.

Acceptance:
- Player can buy seed jars and sell harvested crops through menu.

---

## Phase F — East Path Foraging

### F1. Spawn logic

- Each morning spawn 0–2 mushrooms on valid grass tiles.
- Exclusions: boundary, player tile.

### F2. Harvest forage

- `🍄 -> 🌿` and item added to inventory.

### F3. Rule enforcement

- Block clear/plant/water in East Path with message feedback.

Acceptance:
- Forage loop works independently from farm actions.

---

## Phase G — Back Home + Sleep + Income

### G1. Home alert

- At 02:00 show mandatory sleep alert (`[√] Sleep`).
- Optional earlier prompt variant can include cancel.

### G2. Sleep transition

- Advance to next day.
- Roll new weather.
- Reset daily crop watering flags.

### G3. Income summary screen

- Show day income list, e.g.:
  - `💰 * 200`
  - `🍓 * 20`
  - `🍄 * 1`

Acceptance:
- Day end always resolves through sleep and summary.

---

## 5) Controls Contract (MVP Mode)

- Arrow keys: movement
- `C`: clear
- `P`: plant
- `W`: water
- `H`: harvest
- `T`: trade
- `Esc`: quit/back menu

---

## 6) Testing Plan

## 6.1 Unit tests

- Tile passability and transitions
- Crop growth timing and watering dependency
- Time progression and day rollover
- Weather randomization boundaries
- Trade buy/sell accounting
- Forage spawn validity constraints

## 6.2 Integration tests

- End-to-end farm loop scenario
- Farm ↔ East Path traversal and rule enforcement
- Day-end sleep + income summary flow

## 6.3 Verification commands

- `cargo build`
- `cargo clippy --all-targets --all-features`
- `cargo test`
- `cargo fmt --check`

---

## 7) Delivery Milestones

- **M1** World + movement + UI header
- **M2** Farming actions + crop growth
- **M3** Trading buy/sell menus
- **M4** East Path forage area
- **M5** Sleep/income day loop complete
- **M6** Full verification green + spec compliance pass

---

## 8) Risks and Mitigations

- **Input complexity in terminal**: isolate input mode/state machine and keep menu modes explicit.
- **State drift between maps**: keep single authoritative `GameState` and immutable transition functions.
- **Spec/UI mismatch**: include a spec checklist during final acceptance pass.

---

## 9) Definition of Done

MVP is done when:

1. All core loops in spec are playable in terminal.
2. Farm and East Path rules match documented behavior.
3. Trading + sleep + income are complete.
4. Verification commands pass cleanly.
5. User can run and play using only documented controls.
