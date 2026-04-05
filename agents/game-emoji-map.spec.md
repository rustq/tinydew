# Game Emoji Map

## Tiles (world map)
| Concept / TileType | Emoji | Notes |
|---|---|---|
| Boundary (`Boundary`) | 🌳 | Non-walkable edge/tree |
| Grass (`Grass`) | 🌿 | Default walkable ground |
| Soil (`Soil`) | 🍃 | Tilled soil |
| Crop, immature (`Crop` not mature) | 🌱 | Rendered for any non-mature crop |
| House / Home (`House`) | 🏠 | Non-walkable |
| Path to EastPath (`PathEast`) | 🌿 | Transition tile; rendered same as grass |
| Path to Farm (`PathFarm`) | 🌿 | Transition tile; rendered same as grass |
| Path to Square (`PathSquare`) | 🌿 | Transition tile; rendered same as grass |
| Path to SouthRiver (`PathSouthRiver`) | 🌿 | Transition tile; rendered same as grass |
| SouthRiver gate (`PathSouthRiverGate`) | 🌿 | Transition tile; rendered same as grass |
| Flower tile (`Flower`) | 🌺 | Non-walkable forage/blocker |
| Mushroom tile (`Mushroom`) | 🍄 | Non-walkable forage/blocker |
| Fountain (`Fountain`) | ⛲ | Non-walkable |
| River (`River`) | 🌊 | Non-walkable; fishable |
| River bubble (`RiverBubble`) | 🫧 | Non-walkable; fishable state |
| Wonder (`Wonder`) | 🦋 | Non-walkable (festival/wonder tile) |
| Player tile (`Player`) | 🧑 | TileType has an emoji, but runtime rendering usually overlays entities (see Entities) |

## Crops (produce emoji)
| CropType | Produce emoji |
|---|---|
| Carrot | 🥕 |
| Strawberry | 🍓 |
| Cauliflower | 🥦 |

## Forage
| ForageType | Emoji |
|---|---|
| Mushroom | 🍄 |
| Flower | 🌺 |

## Fish
| FishType | Emoji |
|---|---|
| Common | 🐟 |
| Rare | 🐠 |

## Weather / time icon (header)
| Condition | Emoji |
|---|---|
| Daytime + Sunny | ☀️ |
| Daytime + Cloudy | ⛅ |
| Daytime + Rainy | 🌧 |
| Night (any weather) | 🌙 |

## Entities (map overlay)
| Entity / state | Emoji | Notes |
|---|---|---|
| Player | 🧑 | Drawn on top of the underlying tile |

## UI / inventory affordances
| UI concept | Emoji | Notes |
|---|---|---|
| Money | 💰 | Shown in TUI / `status` as `Money: 💰 $<amount>` (match `ui.spec.md`) |
| Seeds (count) | 🫙 | Line in TUI / `status`: `seeds: 🫙 x<count>` |
