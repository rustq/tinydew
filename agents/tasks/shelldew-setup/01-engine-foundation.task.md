# Task: Engine & World Loop Foundation

**Source Spec:** `agents/shelldew.spec.md`
**Source Plan:** `agents/plan/20260314/shelldew-setup.plan.md`
**Workstream:** W1 - Engine & World Loop
**Milestone:** M1 - Foundations

## Description

Establish core game engine foundation including real-time tick scheduler, input handling, movement system, and initial map rendering. This creates technical backbone that all other systems will build upon.

## Tasks

### 1.1 Project Structure & Crates
- Create Rust crate structure with modules: `engine`, `rendering`, `world`, `input`, `time`
- Set up `Cargo.toml` with necessary dependencies (crossterm for terminal, serde for serialization)
- Define basic module interfaces and visibility

### 1.2 Real-Time Tick Scheduler
- Implement tick-based time system targeting 2.5 seconds per in-game minute
- Create `TimeManager` struct tracking game time (day, hour, minute, season)
- Add tick callback system for game loop updates
- Support pause/resume functionality

### 1.3 Input Handling System
- Implement WASD + arrow key movement input
- Create `InputManager` to capture and process keyboard events
- Add keybinding system for actions (E, F, I, M, T, Esc, Space)
- Ensure non-blocking input for real-time responsiveness

### 1.4 Movement & Collision System
- Implement player position tracking (x, y coordinates)
- Create collision detection against map tiles (water, trees, buildings)
- Add boundary checking for map edges
- Support smooth, responsive movement (no grid-snapping delay)

### 1.5 Map Data Structure
- Define `Tile` enum with `TerrainType` variants (grass, weeds, soil, water, stone, wood, tree, etc.)
- Create `Map` struct with 2D grid of tiles
- Implement map dimensions (initial: ~30x20 for farm area)
- Add map loading and validation

### 1.6 Initial Farm Map
- Create starting farm layout matching spec example:
  - 🌿 grass, 🌾 weeds, 🪨 stone, 🪵 wood, 🏚 old farmhouse, 🌊 river, 🌳 trees
  - Include player starting position
- Store map data in structured format (JSON or TOML for easy iteration)
- Implement map rendering with emoji tiles

### 1.7 Basic Rendering System
- Create terminal renderer using crossterm
- Implement clear screen and cursor positioning
- Render map tiles with emoji characters
- Add support for Unicode 15 emoji baseline
- Include fallback ASCII rendering for debugging

### 1.8 UI Layout Foundation
- Create top status bar placeholder (date, time, weather, money)
- Create bottom help bar placeholder (controls)
- Implement main viewport for map rendering
- Add proper spacing and layout calculations

## Acceptance Criteria

- [ ] Project builds successfully with `cargo build`
- [ ] `cargo clippy` passes without warnings
- [ ] Real-time loop runs at approximately 2.5s per in-game minute
- [ ] WASD and arrow keys move player smoothly
- [ ] Player cannot walk through water, trees, or buildings
- [ ] Player cannot move outside map boundaries
- [ ] Starting farm map matches spec layout exactly
- [ ] Map renders with emoji tiles in terminal
- [ ] UI shows placeholder status and help bars
- [ ] Movement is responsive (no lag or input delay)

## Dependencies

- None (foundation task)

## Estimated Effort

- 4-6 hours

## Notes

- Prioritize responsiveness over complexity
- Keep map data in external files for easy iteration
- Use terminal size detection for responsive viewport
- Document time scale assumptions (1 real hour = 1 game day)
