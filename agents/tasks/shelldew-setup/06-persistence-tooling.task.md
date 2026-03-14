# Task: Persistence & Tooling

**Source Spec:** `agents/shelldew.spec.md`
**Source Plan:** `agents/plan/20260314/shelldew-setup.plan.md`
**Workstream:** W6 - Persistence & Tooling
**Milestone:** M6 - Persistence & Polish

## Description

Implement save/load system, autosave, and development tooling (formatting, linting, testing). This ensures game state can be preserved and codebase remains maintainable.

## Tasks

### 6.1 Save Game Schema Design
- Define `SaveGame` struct containing:
  - Player state (position, inventory, money)
  - World state (maps, tiles, entities)
  - Time state (day, season, time, weather)
  - Farming state (crops, cleared tiles)
  - Social state (NPC relationships, dialogue history)
  - Cat state (position, last petted)
- Add version field for schema migration
- Implement serde serialization

### 6.2 Save File System
- Create save save file structure:
  - `saves/shelldew_save.json` (main save)
  - `saves/autosave.json` (autosave)
  - `saves/backup/` (rotating backups)
- Implement save directory creation
- Add file validation and error handling
- Support multiple save slots (optional for MVP)

### 6.3 Save Implementation
- Implement `save_game()` function:
  - Serialize game state to JSON
  - Write to save file
  - Handle file I/O errors
  - Create backup before overwriting
- Add save confirmation message
- Implement save hotkey (optional for MVP)

### 6.4 Load Implementation
- Implement `load_game()` function:
  - Read save file
  - Deserialize JSON to game state
  - Validate save version
  - Handle missing/invalid saves
- Add load confirmation message
- Implement load on game start (if save exists)

### 6.5 Autosave System
- Implement autosave triggers:
  - At sleep/end of day
  - Before map transitions
  - On critical events (optional)
- Add autosave indicator in UI
- Implement autosave interval (every 5-10 minutes)
- Handle autosave failures gracefully

### 6.6 Save/Load UI
- Add save/load menu (Esc key)
- Implement save slot selection (if multiple slots)
- Display save metadata:
  - Save date/time
  - In-game day/season
  - Playtime
- Add confirmation dialogs
- Handle save/load errors with user feedback

### 6.7 Save State Validation
- Implement save integrity checks:
  - Validate player position is within bounds
  - Validate inventory quantities
  - Validate crop states
  - Validate NPC data
- Add repair functions for corrupted saves
- Log validation errors

### 6.8 Code Formatting Setup
- Add `rustfmt.toml` configuration:
  - Line width: 100 chars
  - Edition: 2024
  - Import style: std
- Configure CI to run `cargo fmt --check`
- Add pre-commit hook for formatting (optional)

### 6.9 Linting Configuration
- Configure `clippy` settings:
  - Allow common warnings
  - Deny specific anti-patterns
  - Set pedantic level
- Add `.clippy.toml` configuration
- Configure CI to run `cargo clippy`

### 6.10 Unit Tests Setup
- Create test module structure:
  - `tests/engine_tests.rs`
  - `tests/farming_tests.rs`
  - `tests/world_tests.rsver`
- Add basic test utilities
- Configure test discovery

### 6.11 Engine Tests
- Implement tests for:
  - Time management (tick calculations)
  - Input handling (key mapping)
  - Movement (collision detection)
  - Map rendering (tile conversion)
- Add edge case tests

### 6.12 Farming Tests
- Implement tests for:
  - Tile state transitions
  - Crop growth logic
  - Tool interactions
  - Inventory operations
- Add seasonal validation tests

### 6.13 Save/Load Tests
- Implement tests for:
  - Save serialization
  - Load deserialization
  - Version migration
  - Corrupted save handling
- Add round-trip tests (save → load → verify)

### 6.14 Integration Tests
- Create integration test scenarios:
  - Full farming loop (till → plant → water → harvest)
  - Day transition (sleep → new day)
  - Season progression (28 days)
  - Save/load cycle

### 6.15 CI/CD Configuration
- Create CI workflow:
  - Run `cargo fmt --check`
  - Run `cargo clippy`
  - Run `cargo test`
  - Run `cargo build --release`
- Add GitHub Actions or similar
- Configure test matrix (stable, nightly)

### 6.16 Build Scripts
- Create `Makefile` with targets:
  - `make build` - debug build
  - `make release` - release build
  - `make test` - run tests
  - `make lint` - run clippy
  - `make fmt` - format code
  - `make clean` - clean artifacts
- Add script documentation

### 6.17 Development Documentation
- Create `DEVELOPMENT.md` with:
  - Build instructions
  - Test running instructions
  - Code style guidelines
  - Debugging tips
- Update `README.md` with development section
- Add contribution guidelines

## Acceptance Criteria

- [ ] `cargo build` succeeds
- [ ] `cargo clippy` passes without warnings
- [ ] `cargo test` passes all tests
- [ ] `cargo fmt --check` passes
- [ ] Game saves to file correctly
- [ ] Game loads from file correctly
- [ ] Autosave triggers at sleep/end of day
- [ ] Save/load preserves all game state
- [ ] Corrupted saves are handled gracefully
- [ ] Save/load UI works correctly
- [ ] Unit tests cover core systems
- [ ] Integration tests cover key workflows
- [ ] CI pipeline runs successfully
- [ ] Makefile provides all necessary targets

## Dependencies

- Task 01: Engine & World Loop
- Task 02: Farming System
- Task 03: Environment & Atmosphere
- Task 04: Fishing & Cat System
- Task 05: Town & Social System

## Estimated Effort

- 6-8 hours

## Notes

- Per spec: save/load is MVP requirement
- Use serde for serialization (battle-tested)
- Implement versioned saves for future compatibility
- Autosave should be transparent to player
- Tests should be fast and focused
- CI should catch regressions early
- Documentation should help future contributors
