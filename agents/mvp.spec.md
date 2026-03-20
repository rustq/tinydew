# CLI Farm Game Shelldew --- MVP Specification

A minimal terminal-based farming simulation game inspired by Stardew
Valley.

The game runs entirely in a CLI environment and uses emoji tiles
as the world representation.

------------------------------------------------------------------------

# 1. Core Design Goals

The MVP focuses on three core systems:

1. Farming interaction
2. Crop growth
3. Trading system

Design principles:

- Minimal UI
- Emoji-based map
- Simple keyboard controls
- Expandable world

------------------------------------------------------------------------

# 2. Game World

The world currently contains two areas.

Farm (main gameplay) │ └── East Path (foraging dead-end)

Future expansions may include:

- River
- Forest
- Mountain path
- Town

------------------------------------------------------------------------

# 3. Map System

## 3.1 Farm Map

The farm is an 8×8 tile grid. The player is not stored in the map data; they are drawn at the current position `(player_x, player_y)` from game state. Map cells use Grass (or PathEast at the exit) where the player can stand. Default player start is (3, 3).

Example starting map (🧑 at start):

🌳🌳🌳🌳🌳🌳🌳🌳\
🌳🌿🌿🌿🌿🌿🌿🌳\
🌳🌿🏠🌿🌿🌿🌿🌳\
🌳🌿🌿🌿🧑🌿🌿🌳\
🌳🌿🌿🌿🌿🌿🌿🌳\
🌳🌿🌿🌿🌿🌿🌿🌿\
🌳🌿🌿🌿🌿🌿🌿🌳\
🌳🌳🌳🌳🌳🌳🌳🌳

(The exit to East Path is bottom-right; transition tile may display as 🌿 for grid alignment.)

------------------------------------------------------------------------

## 3.2 Tile Types

 Emoji/Symbol  Meaning
 ------------- -------------------
 🌳            Boundary tree
 🌿            Grass / weeds (path transition tiles may also display as 🌿)
 ▪️            Cleared soil
 🌱            Growing crop
 🥕🍓🥦🌺     Mature crops
 🧑            Player (drawn from state; not a map tile)
 🏠            Farm house
 (PathEast)    Path to East area — logic only; display as 🌿 for alignment
 (PathFarm)   Path back to Farm (East Path only) — logic only; display as 🌿

------------------------------------------------------------------------

# 4. UI Layout

The CLI UI consists of two parts.

## Header

🌸 Spring Day 4 🌧 Rain 08:20

Displays:

- Season
- Day
- Weather
- Time

------------------------------------------------------------------------

## Map

🌳🌳🌳🌳🌳🌳🌳🌳\
🌳🌿🌿🌿🌿🌿🌿🌳\
🌳🌿🏠🌿🌿🌿🌿🌳\
🌳🌿▪️🧑▪️🌿🌿🌳\
🌳🌿🌱🌱🍓🌿🌿🌳\
🌳🌿▪️▪️▪️🌿🌿🌿\
🌳🌿🌿🌿🌿🌿🌿🌳\
🌳🌳🌳🌳🌳🌳🌳🌳

## Terminal display (macOS / raw mode)

- Use CRLF (`\r\n`) for line endings so each line starts at column 0 and the map does not render as a staircase.
- Player is drawn only from `(player_x, player_y)`; map data does not contain a Player tile.
- Use single-width or consistent-width tiles: 🏠 for house; path transition tiles may be displayed as 🌿 so the grid aligns with 🌳.

The UI intentionally does not show:

- Inventory
- Money

All feedback appears as action messages.

------------------------------------------------------------------------

# 5. Time System

Each action advances time.

1s = 5 minutes

Typical day:

06:00 → 22:00

At night the weather icon changes to:

🌙

Regardless of the daytime weather.

Example:

🌸 Spring Day 4 🌙 22:10

After night ends:

Next Day → new weather generated

------------------------------------------------------------------------

# 6. Movement

Movement uses arrow keys.

↑ move up\
↓ move down\
← move left\
→ move right

Movement restrictions:

- Cannot walk into 🌳
- Can walk on grass, soil, or crops

------------------------------------------------------------------------

# 7. Farming Interaction

Actions target adjacent tiles and do not depend on facing direction.
For default action commands, choose target tile from adjacent neighbors using deterministic priority:
1. Up
2. Right
3. Down
4. Left

All the interaction done will show message XXX Done! at bottom ui

Controls:

 Key Action
 ----- -------------
 C Clear weeds
 P Plant
 W Water
 H Harvest
 T Trade

------------------------------------------------------------------------

## Clear

🌿 → ▪️

------------------------------------------------------------------------

## Plant

▪️ → 🌱

Plant consumes one seed.

------------------------------------------------------------------------

## Water

Water helps crops grow (if do not water it will not grow).

------------------------------------------------------------------------

## Harvest

🥕🍓🥦🌺 → 🌿

Harvested crops are stored internally.

------------------------------------------------------------------------

# 8. Crops

All MVP crops are Spring crops.

 Crop Emoji Growth Time
 ------------- ------- -------------
 Carrot 🥕 4 days
 Strawberry 🍓 8 days
 Cauliflower 🥦 12 days
 Rhubarb 🌺 16 days

------------------------------------------------------------------------

## Crop Lifecycle

🌿 weeds\
↓ clear\
▪️ soil\
↓ plant\
🌱 growing\
↓ time\
🥕🍓🥦🌺 mature\
↓ harvest\
🌿soil

------------------------------------------------------------------------

# 9. Seeds

Seeds are purchased as jars.

🫙 Carrot Seed\
🫙 Strawberry Seed\
🫙 Cauliflower Seed\
🫙 Rhubarb Seed

------------------------------------------------------------------------

# 10. Trading System

Open shop:

T

Menu navigation:

↑ ↓ move\
Enter confirm

------------------------------------------------------------------------

## Shop Menu

Shop

\[√\] Buy 🫙 Carrot Seed\
\[ \] Buy 🫙 Strawberry Seed\
\[ \] Buy 🫙 Cauliflower Seed\
\[ \] Buy 🫙 Rhubarb Seed\
\[ \] Sell Crops\
\[ \] Exit

------------------------------------------------------------------------

## Sell Menu

Sell (Show the things you have)

\[√\] Sell 🥕 Carrot\
\[ \] Sell 🍓 Strawberry\
\[ \] Sell 🥦 Cauliflower\
\[ \] Sell 🌺 Rhubarb\
\[ \] Back

------------------------------------------------------------------------

# 11. Weather

Weather is randomized each day.

Possible weather:

☀️ Sunny\
🌧 Rain\
☁️ Cloudy

At night:

🌙 overrides all weather icons.

If weather is Rain, the

------------------------------------------------------------------------

# 12. East Path Area

A small foraging dead-end area connected to the farm.

## Map

East Path is a dead-end; the player is drawn at `(player_x, player_y)`. Entry from farm places player at (1, 2). Path back to farm is leftmost cell (PathFarm); it may display as 🌿.

🌳🌳🌳🌳🌳🌳🌳🌳🌳🌳🌳\
🌳🌿🌿🌿🌿🌿🌿🌿🌿🌿🌳\
🌿🧑🌿🌿🌿🌿🌿🌿🌿🍄🌳\
🌳🌳🌳🌳🌳🌳🌳🌳🌳🌳🌳

------------------------------------------------------------------------

# 13. Back home

A alert will show in UI and could choose a choice to do

## Alert

Home

\[√\] Sleep\
\[ \] Cancel\


every 02:00 the Alert will show and only one choice

Home

\[√\] Sleep\

## Sleep

if choosed sleep the ui will change to the todays income like

Income this day

💰 * 200
🍓 * 20
🍄 * 1

------------------------------------------------------------------------

## Rules

Farming actions are disabled here.

Not allowed:

clear\
plant\
water

Grass cannot become soil.

------------------------------------------------------------------------

## Foraging

Forage items can spawn:

🍄

Harvest rule:

🍄 → 🌿

------------------------------------------------------------------------

## Spawn Logic

Each morning:

0--2 mushrooms spawn

Spawn rules:

- Only on 🌿 tiles
- Not on boundary
- Not on player

------------------------------------------------------------------------

# 14. Core Gameplay Loop

Move around farm\
↓\
Clear weeds\
↓\
Plant crops\
↓\
Water crops\
↓\
Wait for growth\
↓\
Harvest crops\
↓\
Sell crops\
↓\
Buy more seeds

Optional exploration:

Visit East Path\
↓\
Find forage items\
↓\
Harvest mushrooms

Back home\
↓\
Show Income\
↓\
Next day


------------------------------------------------------------------------

# 14. MVP Scope

Included:

- 8×8 farm
- 4 crops
- crop growth system
- weather
- time progression
- trading menu
- East Path forage area
- Back home
- Show income

Excluded:

- animals
- crafting
- energy system
- NPCs
- town
- buildings
- automation

------------------------------------------------------------------------

# 15. Future Expansion

Potential systems:

- Summer crops
- Autumn crops
- Fishing
- Tools
- Irrigation
- NPC town
- Crafting
- Automation
- Multiple regions

------------------------------------------------------------------------

# 16. Design Philosophy

The game follows three key principles.

### Minimal UI

Only header + map are displayed.

### Emoji World

All elements are emoji tiles.

### Simple Controls

Arrow keys → movement\
C P W H → farm actions\
T → trading

------------------------------------------------------------------------

End of Specification