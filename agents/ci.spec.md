# CI Spec

## Status
Implemented.

## Workflows

### CI (`ci.yml`)
- **Trigger**: Push to any branch, PR to `main`.
- **Runner**: `ubuntu-latest`.
- **Steps**:
  1. Checkout with submodules (`actions/checkout@v4`, `submodules: true`).
  2. Install stable Rust toolchain with clippy (`dtolnay/rust-toolchain@stable`).
  3. `cargo build` — compile the project.
  4. `cargo test -- --test-threads=1` — run tests single-threaded to avoid state conflicts.

### UI (`ui.yml`)
- **Trigger**: Push to any branch, PR to `main`.
- **Runner**: `ubuntu-latest`.
- **Steps**:
  1. Checkout (`actions/checkout@v4`).
  2. Install stable Rust toolchain (`dtolnay/rust-toolchain@stable`).
  3. `cargo test initial_farm_ui -- --nocapture` — run the farm UI snapshot test with visible output.

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
     - `cargo run -- do fish up` → `cargo run -- status`
  6. Each step uses a descriptive `name` label (e.g., "Move Down", "Status after Move Down") for readable CI output.

## Notes
- Tests run single-threaded (`--test-threads=1`) because game state is shared and not thread-safe.
- The UI workflow captures the initial farm rendering as a visual smoke test.
- The Showcase workflow provides a visual walkthrough of all CLI actions in CI logs.
- Submodules are required for CI build (piano sound samples dependency).
