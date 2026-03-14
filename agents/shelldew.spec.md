# Shelldew CLI Emoji Farm Game Spec

## Working Title: **Shelldew**

## 1. Game Summary

Shelldew is a real-time CLI farming and town-life game rendered primarily with emoji. The player starts on a rough, partially overgrown farm and can move freely using keyboard input, clear land, till soil, water crops, fish by the river, pet the cat, visit town, chat with villagers, and buy or sell goods.

The game features:

* emoji-first map rendering
* real-time keyboard movement
* four seasons
* random daily weather
* farming and fishing
* cat interaction
* town shopping and social interaction
* approximately **1 real-world hour per in-game day**

This game should feel cozy, readable, expressive, and alive inside the terminal.

---

## 2. Design Goals

### Primary Goals

* Build a **cozy terminal life sim** with strong emoji presentation.
* Make the world feel readable and charming using mostly emoji tiles.
* Support a satisfying loop of:

  * exploration
  * farming
  * fishing
  * shopping
  * socializing
  * seasonal progression

### Important Constraints

* No stamina system
* No fog weather
* Player movement should be direct and responsive
* The starting farm should feel **messy, natural, and undeveloped**, not already optimized

---

## 3. Core Gameplay Pillars

### Farming

* clear debris
* till land
* plant crops
* water crops
* harvest crops
* sell crops or use them later

### Life Sim

* walk around the farm
* visit town
* talk to villagers
* shop and sell
* pet the cat
* enjoy weather and season changes

### Exploration

* move between farm, river, home, town, and shops
* discover daily variation through weather and schedules

---

## 4. Time System

### Day Length

* **1 in-game day ≈ 1 real-world hour**

### Suggested Time Scale

* 1 in-game minute = 2.5 real seconds
* 24 in-game hours = about 60 real minutes

### Time Phases

* 🌅 Morning
* ☀️ Day
* 🌇 Evening
* 🌙 Night

### Time Effects

* shops open and close by time
* NPC locations change by time
* visuals and atmosphere change by time
* fishing tables may depend on time

### No Stamina

There is **no stamina / energy mechanic**.

This means:

* player actions are not limited by energy cost
* the pacing is governed by **time**, not exhaustion
* challenge comes from planning, travel time, shop hours, crop timing, and weather

Optional late-night penalties may still exist, but they should be lightweight and not tied to a stamina bar.

---

## 5. Seasons

The game supports four seasons:

* 🌸 Spring
* ☀️ Summer
* 🍂 Autumn
* ❄️ Winter

### Seasonal Changes

Each season affects:

* terrain visuals
* crop availability
* tree visuals
* weather probabilities
* fishing tables
* town decoration
* ambient map feeling

### Suggested Season Length

* 28 in-game days per season

---

## 6. Weather System

### Supported Weather Types

* ☀️ Sunny
* ☁️ Cloudy
* 🌧️ Rainy
* ⛈️ Stormy
* ❄️ Snowy (winter only)

### Not Supported

* No fog weather

### Weather Effects

* Rain automatically waters outdoor crops for the day
* Storm may increase fishing rarity or change NPC routines
* Snow changes ground visuals and disables most outdoor crop growth
* Weather is rolled once per day at sleep / day start

---

## 7. World Structure

### Required Areas

* Farm
* Farmhouse interior
* River / fishing area
* Town
* Shop interior
* Saloon / social area

### Optional Future Areas

* forest
* mountain
* beach
* mine
* greenhouse

---

## 8. Starting Farm Design

The starting farm should feel rough and undeveloped, similar to this:

```text
🌿🌿🌿🌿🌿🌿🌿🌿
🌿🌾🌾🌿🪨🌿🌾🌿
🌿🌿🏚🌿🌿🌿🌿🌿
🌿🪵🌿🌿🌾🌿🪨🌿
🌿🌿🌿🧑‍🌾🌿🌿🌿🌿
🌊🌊🌊🌊🌊🌊🌊🌊
🌿🌿🌳🌿🌿🌳🌿🌿
🌿🌿🌿🌿🌿🌿🌿🌿
```

### Meaning of Starting Tiles

* 🌿 grass / open ground
* 🌾 weeds / overgrowth
* 🪨 stone
* 🪵 wood / log
* 🏚 old farmhouse
* 🌊 river
* 🌳 trees
* 🧑‍🌾 player

### Starting Farm Intent

The opening farm should communicate:

* this place needs work
* there is room for gradual transformation
* the player begins with a humble setup
* the river is immediately visible and invites fishing
* the farm is small enough to understand quickly, but expandable later

### Early-Game Expectations

At the start, the player should be able to:

* walk around the farm
* clear weeds, wood, and rocks
* till a few tiles
* water crops
* access the river
* return to the farmhouse
* head toward town soon after

---

## 9. Emoji Rendering Principles

The game should use emoji as the main visual system whenever possible.

### Terrain Emoji

* 🌿 grass
* 🌾 weeds
* 🌱 tilled soil
* 🌊 water
* 🧊 frozen water
* 🪨 stone
* 🪵 wood
* 🌳 tree
* 🌲 winter tree
* 🍄 mushroom patch
* 🌉 bridge
* 🏚 old house
* 🏡 upgraded house
* 🚪 door
* 🛤️ path

### Crop Emoji

* 🍓 strawberry
* 🌽 corn
* 🍅 tomato
* 🎃 pumpkin
* 🥕 carrot
* 🍆 eggplant
* 🫐 blueberry

### Social / Town Emoji

* 🏪 shop
* 🍻 saloon
* 🏥 clinic
* 🏛️ town hall
* 🏫 museum
* 🧺 market stall
* 💬 talk prompt
* 💰 commerce prompt

### Character Emoji

* 🧑‍🌾 player
* 🐱 cat
* 👩 👨 🧓 🧒 NPC types
* 🐟 🐠 🐡 fish visuals

### UI Emoji

* 📅 date
* ⏰ time
* 💰 money
* 🌦️ weather
* 🎒 inventory
* 🎣 fishing
* 🐾 pet interaction
* 🗨️ dialogue
* 🛒 shopping

---

## 10. Controls

### Movement

* `W A S D` to move
* arrow keys optional

### Main Actions

* `E` interact / talk / inspect / enter
* `F` use equipped tool
* `1..9` select tool or item slot
* `I` inventory
* `M` map
* `T` social/chat log
* `Esc` cancel / menu
* `Space` confirm / context action

### Tool Actions

* hoe: till land
* watering can: water crops
* fishing rod: fish near water
* axe: chop wood
* pickaxe: break stones

---

## 11. Core Actions

### Walking

* real-time tile movement
* collision-aware
* smooth and responsive

### Clearing

* remove weeds
* break stones
* chop wood
* clear space for farm expansion

### Tilling

* use hoe on valid ground
* convert land into plantable soil

### Watering

* water planted tiles
* rain can replace manual watering for that day

### Planting

* use seeds on tilled ground
* season restrictions apply

### Harvesting

* collect mature crops
* send to inventory

### Fishing

* stand next to fishable water
* cast with fishing rod
* catch result depends on:

  * season
  * weather
  * time
  * location

### Petting the Cat

* once per day if adjacent
* gives emotional flavor and affection progression

### Talking to NPCs

* interact in town or other areas
* NPC dialogue depends on:

  * season
  * weather
  * time
  * relationship level

### Shopping / Selling

* buy seeds, supplies, food, and other goods
* sell fish, crops, and forage

---

## 12. Cat System

The cat is an important cozy feature.

### Requirements

* cat exists from early game
* can be found on the farm or near the house
* can be petted once per day
* may move among a small set of tiles
* uses emoji reactions or short messages

### Example Reactions

* `You pet the cat. 😺`
* `The cat purrs. 🐾`
* `The cat stretches in the rain.`
* `The cat watches the river.`

Optional future additions:

* affection score
* simple daily routines
* small gift events

---

## 13. Farming System

### Crop Stages

* seed
* sprout
* growing
* mature

### Growth Rules

A crop grows daily if:

* it is planted in valid soil
* it is in season
* it was watered that day, unless rain covered it

### Seasonal Crop Examples

#### Spring

* 🍓 strawberry
* 🥕 carrot
* 🥔 potato

#### Summer

* 🌽 corn
* 🍅 tomato
* 🫐 blueberry

#### Autumn

* 🎃 pumpkin
* 🍆 eggplant
* 🍠 yam

#### Winter

* limited or no outdoor farming
* winter shifts focus toward fishing, town life, and other activities

---

## 14. Fishing System

### Fishing Locations

* river
* pond if added later
* seasonal special spots optional later

### Catch Factors

* time of day
* season
* weather
* location

### MVP Fishing Flow

1. stand near water
2. equip rod
3. press action
4. short delay
5. determine catch
6. show result

### Example Results

* `🎣 You caught a 🐟 River Fish!`
* `🎣 You caught a 🐠 Bright Trout!`
* `🎣 Nothing bit.`

---

## 15. Town and Social Layer

The town should feel useful and alive.

### Required Town Features

* general store
* saloon / social building
* town square
* several villagers
* buy and sell functionality

### Required Town Activities

* shopping
* selling
* chatting with NPCs
* casual social visits
* optional evening saloon visits

### Shop Functions

* browse inventory
* buy items
* sell inventory items
* show prices clearly
* seasonal stock changes allowed

### Example Store Categories

* seeds
* crop goods
* fishing supplies
* food
* gifts
* utility items

---

## 16. NPC Social System

Each NPC should have:

* a name
* one or more map locations
* a simple daily schedule
* dialogue pool
* friendship / familiarity value

### Dialogue Factors

* weather
* season
* time of day
* recent events
* shop hours or routines

### Social Feel

Dialogue should be:

* short
* readable
* flavorful
* cozy
* occasionally humorous

---

## 17. UI Layout

### Top Status Bar

```text
📅 Spring 03   ⏰ 07:20   🌧️ Rainy   💰 120g
```

### Center

* emoji tile map

### Bottom Help Bar

```text
[WASD] Move  [E] Interact  [F] Use Tool  [I] Inventory  [T] Chat  [Esc] Menu
```

### No Stamina UI

Do not show any stamina, energy, or exhaustion meter.

---

## 18. Save System

The game must support:

* save to file
* load from file
* autosave at sleep/end of day
* restore full world state

### Save Data Includes

* player position
* inventory
* money
* current day/season/time
* weather
* crop states
* cleared farm tiles
* NPC relationship values
* cat state
* town inventory / stock if needed

---

## 19. Minimum Viable Product

### MVP Must Include

* emoji farm rendering
* real-time keyboard movement
* starting farm similar to the provided rough layout
* four seasons
* daily random weather
* no fog weather
* no stamina system
* tilling
* watering
* planting and harvesting
* fishing
* cat petting
* town with shopping and selling
* NPC chatting/social interaction
* save/load
* one-hour day length

### MVP Can Skip

* romance
* festivals
* mines
* combat
* advanced crafting
* furniture placement
* multiplayer
* complex fishing minigame

---

## 20. Recommended Development Milestones

### Milestone 1

* map renderer
* input
* movement
* farm map
* time progression

### Milestone 2

* clearing debris
* tilling
* watering
* planting
* harvesting

### Milestone 3

* daily weather
* four seasons
* fishing system

### Milestone 4

* town
* shop buy/sell
* NPC dialogue
* cat interaction

### Milestone 5

* polish
* seasonal visuals
* better dialogue variety
* improved map transitions
* save/load robustness

---

## 21. Example Early-Game Session

```text
06:00 You wake up in the old farmhouse 🏚
06:05 Weather today: 🌧️ Rainy
06:10 You walk outside onto the messy farm
06:15 You clear a 🪵 log
06:20 You break a 🪨 stone
06:30 You till a few 🌿 tiles into 🌱
06:40 You visit the river and fish 🎣
07:00 You pet the cat 🐱
07:20 You walk to town
07:40 You buy seeds at the shop 🏪
08:00 You chat with a villager 💬
08:20 You return to the farm and plant seeds
22:30 You sleep and the next day begins
```

---

## 22. Product Definition

Shelldew should feel like a terminal-native, emoji-rich, cozy farming life sim where:

* the farm starts rough and gradually transforms
* time matters more than stamina
* weather and seasons shape the rhythm
* the town feels worth visiting
* the world is expressive through emoji
* the CLI presentation is part of the charm, not a limitation
