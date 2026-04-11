# CI Spec

## Status
Implemented.

## Workflows

### CI (`ci.yml`)
- **Trigger**: Push to any branch, PR to `main`.
- **Runner**: `ubuntu-latest`.
- **Steps**:
  1. Checkout repository (`actions/checkout@v4`).
  2. Install stable Rust toolchain with clippy (`dtolnay/rust-toolchain@stable`).
  3. `cargo build` — compile the project.
  4. `cargo test -- --test-threads=1` — run tests single-threaded to avoid state conflicts.

### UI (`ui.yml`)
- **Trigger**: Push to any branch, PR to `main`.
- **Runner**: `ubuntu-latest`.
- **Steps**:
  1. Checkout (`actions/checkout@v4`).
  2. Install stable Rust toolchain (`dtolnay/rust-toolchain@stable`).
  3. `cargo test initial_farm_ui -- --nocapture` — run the farm UI regression test with visible output.

### Showcase (`show-case.yml`)
- **Trigger**: Push to any branch, PR to `main`.
- **Runner**: `ubuntu-latest`.
- **Purpose**: Demonstrate all CLI actions and print the game status after each one as a showcase.
- **Steps**:
  1. Checkout (`actions/checkout@v4`).
  2. Install stable Rust toolchain (`dtolnay/rust-toolchain@stable`).
  3. `cargo build` — compile the project.
  4. Print initial status:
     - `cargo run -- status`
  5. Run each action followed by `cargo run -- status` to show the result:
     **Basic actions at Farm (start at (3,3)):**
     - `cargo run -- do move down` → `cargo run -- status`
     - `cargo run -- do move left` → `cargo run -- status`
     - `cargo run -- do move right` → `cargo run -- status`
     - `cargo run -- do move up` → `cargo run -- status`
     - `cargo run -- do clear up` → `cargo run -- status`
     - `cargo run -- do plant up` → `cargo run -- status`
     - `cargo run -- do water up` → `cargo run -- status`
     - `cargo run -- do buy seed` → `cargo run -- status`
     - `cargo run -- do harvest up` → `cargo run -- status`
     - `cargo run -- do sell 🍓` → `cargo run -- status`
     - `cargo run -- do sell 🍄` → `cargo run -- status`
     **Navigate Farm → EastPath (from (3,3) to PathEast (7,5)):**
     - `cargo run -- do move down` (→ (3,4))
     - `cargo run -- do move down` (→ (3,5))
     - `cargo run -- do move right` (→ (4,5))
     - `cargo run -- do move right` (→ (5,5))
     - `cargo run -- do move right` (→ (6,5))
     - `cargo run -- do move right` (→ PathEast (7,5) → transition to EastPath (1,2))
     - `cargo run -- status` *(show EastPath)*
     **Navigate EastPath → Square (from (1,2) to PathSquare (5,0)):**
     - `cargo run -- do move right` (→ (2,2))
     - `cargo run -- do move right` (→ (3,2))
     - `cargo run -- do move right` (→ (4,2))
     - `cargo run -- do move right` (→ (5,2))
     - `cargo run -- do move up` (→ (5,1))
     - `cargo run -- do move up` (→ PathSquare (5,0) → transition to Square (4,3))
     - `cargo run -- status` *(show Square with fountain and pre-placed flower)*
     **Navigate Square → EastPath → SouthRiver:**
     - `cargo run -- do move down` (→ PathSquare (4,4) → transition to EastPath (5,1))
     - `cargo run -- do move left` (→ (4,1))
     - `cargo run -- do move left` (→ (3,1))
     - `cargo run -- do move left` (→ (2,1))
     - `cargo run -- do move down` (→ (2,2))
     - `cargo run -- do move down` (→ PathSouthRiver (2,3) → transition to SouthRiver (2,1))
     - `cargo run -- status` *(show SouthRiver with river tiles)*
     **Fish at SouthRiver:**
     - `cargo run -- do fish down` → `cargo run -- status`
     **Navigate SouthRiver → EastPath → Farm:**
     - `cargo run -- do move up` (→ PathSouthRiverGate (2,0) → transition to EastPath (2,2))
     - `cargo run -- do move left` (→ (1,2))
     - `cargo run -- do move left` (→ PathFarm (0,2) → transition to Farm (6,5))
     - `cargo run -- status` *(back at Farm)*
     **Sleep and day transition:**
     - `cargo run -- do sleep` → `cargo run -- status` *(show day 2 state)*
  6. Each step uses a descriptive `name` label (e.g., "Move Down", "Status after Move Down", "Navigate to Square", "Status at SouthRiver") for readable CI output.

### Showcase — Regions (`regions-showcase` job)
- **Purpose**: Demonstrate region traversal across all four regions in a single run.
- **DB**: `/tmp/tinydew-regions.sqlite` (isolated).
- **Steps**:
  1. Build the project.
  2. **Farm**: Show initial status at Farm (Day 1).
  3. **Farm → EastPath**: Navigate from (3,3) through (6,5) to PathEast (7,5), transition to EastPath (1,2). Show status.
  4. **EastPath → Square**: Navigate from (1,2) through (5,2) to PathSquare (5,0), transition to Square (4,3). Show status.
  5. **Square → EastPath → SouthRiver**: Transition back to EastPath (5,1), navigate to PathSouthRiver (2,3), transition to SouthRiver (2,1). Show status.

### Showcase — Grow (`grow-showcase` job)
- **Purpose**: Demonstrate selective watering and its effect on crop growth across day transition.
- **DB**: `/tmp/tinydew-grow.sqlite` (isolated).
- **Steps**:
  1. Build the project.
  2. Buy 5 seeds (start with 1 → total 6).
  3. Navigate along column x=3 from y=1 to y=6, clearing and planting at (4,1), (4,2), (4,3), (4,4), (4,5), (4,6). Show status.
  4. Navigate back and water only (4,1), (4,3), (4,5) — skip (4,2), (4,4), (4,6).
  5. Sleep to Day 2. Show status — watered crops matured, unwatered crops remain seedlings.

## Notes
- Tests run single-threaded (`--test-threads=1`) because game state is shared and not thread-safe.
- The UI workflow captures the initial farm rendering as a visual smoke test.
- The Showcase workflow provides a visual walkthrough of all CLI actions in CI logs.
