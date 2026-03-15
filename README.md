# 🌾 shelldew

A cozy emoji-first CLI farming simulator built with Rust.

## 🎮 Features

- **Real-time game loop** with 2.5 seconds per in-game minute
- **WASD movement** with collision detection
- **Emoji-based rendering** with ASCII fallback
- **Dynamic seasons** (Spring, Summer, Autumn, Winter)
- **Weather system** (Sunny, Cloudy, Rainy, Stormy, Snowy)
- **Farming mechanics** (tilling, planting, watering, harvesting)
- **Tool system** (Hoe, Watering Can, Axe, Pickaxe, Fishing Rod)
- **Inventory management** with item stacking
- **Crop growth system** with seasonal validation

## 🚀 Getting Started

### Prerequisites

- Rust 2024 edition
- Cargo (Rust package manager)

### Installation

```bash
# Clone repository
git clone https://github.com/meloalright/shelldew.git
cd shelldew

# Build project
cargo build

# Run game
cargo run
```

## 🎮 Controls

- **WASD / Arrow Keys** - Move player
- **E** - Use current tool
- **1-9** - Select tool slot
- **Esc** - Quit game

## 🛠️ Tools

1. **Hoe** (⛏️) - Till grass into soil
2. **Watering Can** (💧) - Water crops
3. **Axe** (🪓) - Clear wood
4. **Pickaxe** (⛏️) - Clear stones
5. **Fishing Rod** (🎣) - Fish near water

## 🌾 Crops

| Crop | Season | Growth Time |
|-------|--------|-------------|
| Strawberry | Spring, Autumn | 3 days |
| Corn | Summer | 4 days |
| Tomato | Summer | 3 days |
| Pumpkin | Autumn | 5 days |
| Carrot | Spring, Autumn | 2 days |
| Eggplant | Autumn | 4 days |
| Blueberry | Summer | 3 days |

## 🌦️ Seasons & Weather

- **Spring** (🌸) - Fresh grass, blooming trees
- **Summer** (☀️) - Vibrant colors, full trees
- **Autumn** (🍂) - Orange/brown palette, falling leaves
- **Winter** (❄️) - White ground, bare trees

**Weather Effects:**
- Rain automatically waters crops
- Storms increase fishing chances
- Snow disables outdoor crop growth

## 🎮 Running the Game

### Basic Run

```bash
# Navigate to project directory
cd shelldew

# Run the game
cargo run
```

### Initial UI Test

When you first run the game, you should see:

```
┌─────────────────────────────────────────────────────────────┐
│ Day 1 06:01 | 🌸 | Weather: ☀️ | Money: 💰0              │
├─────────────────────────────────────────────────────────────┤
│ 🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌊🌿🌿🌿 │
│ 🌿🏚🏚🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌊🌿🌿🌿 │
│ 🌿🏚🏚🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌿🌊🌿🌿🌿 │
│ 🌿🌿🌿🌿🌳🌳🌿🌿🌿🌿🌿🌿🌿🌊🌿🌿🌿 │
│ 🌿🌿🌿🌿🌳🌿🌿🌿🌿🌿🌿🌿🌿🌊🌿🌿�� │   (more map rows...)
├─────────────────────────────────────────────────────────────┤
│ WASD: Move | Esc: Quit                                       │
└─────────────────────────────────────────────────────────────┘
```

**UI Elements:**
- **Top status bar** - Shows day, time, season, weather, and money
- **Main viewport** - Displays the game map with emoji tiles
- **Bottom help bar** - Shows available controls

**Map Legend:**
- 😐 - Player
- 🌿 - Grass
- 🏚 - Farmhouse
- 🌳 - Tree
- 🪨 - Stone
- 🪵 - Wood
- 🌾 - Weeds
- 🌊 - River

### Testing Controls

1. **Movement Test**
   - Press W/A/S/D or Arrow keys
   - Player should move smoothly
   - Player cannot walk through water, trees, or buildings

2. **Time Test**
   - Watch the time in the status bar
   - Time should advance every 2.5 seconds
   - Day 1 06:01 → 06:02 → 06:03...

3. **Exit Test**
   - Press Esc key
   - Game should exit cleanly

## 🧪 Development

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Run Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test
```

### Format & Lint

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

## 📁 Project Structure

```
shelldew/
├── src/
│   ├── main.rs          # Entry point
│   ├── engine/          # Game engine & main loop
│   ├── rendering/       # Terminal rendering
│   ├── world/           # Map, tiles, player
│   ├── input/           # Input handling
│   ├── time/            # Time management
│   ├── farming/          # Crop system
│   ├── tools/           # Tool definitions
│   └── inventory/        # Inventory management
├── agents/
│   ├── shelldew.spec.md   # Game specification
│   ├── plan/              # Development plans
│   ├── tasks/             # Task breakdowns
│   ├── constitution.md     # Agent guidelines
│   └── README.md         # This file
└── Cargo.toml            # Dependencies
```

## 🎯 License

MIT License - See LICENSE file for details

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📞 Support

For issues and questions, please open an issue on GitHub.
