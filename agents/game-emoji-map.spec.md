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
| Mushroom tile (`Mushroom`) | 🍄 | Non-walkable forage/blocker |
| Fountain (`Fountain`) | ⛲ | Non-walkable |
| Slide (`Slide`) | 🛝 | Non-walkable |
| River (`River`) | 🌊 | Non-walkable; fishable |
| River bubble (`RiverBubble`) | 🫧 | Non-walkable; fishable state |
| Wonder (`Wonder`) | 🦋 | Non-walkable (festival/wonder tile) |
| Piano (`Piano`) | 🎹 | Non-walkable |
| Player tile (`Player`) | 🧑 | TileType has an emoji, but runtime rendering usually overlays entities (see Entities) |

## Crops (produce emoji)
| CropType | Produce emoji |
|---|---|
| Carrot | 🥕 |
| Strawberry | 🍓 |
| Cauliflower | 🥦 |
| Flower | 🌺 |

## Forage
| ForageType | Emoji |
|---|---|
| Mushroom | 🍄 |

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
| Guest | 👧 | Drawn only when the guest is visible on the current map; if both overlap, the active control target decides which emoji shows |

## UI / inventory affordances
| UI concept | Emoji | Notes |
|---|---|---|
| Money | 💰 | Rendered in snapshot as `money: 💰 $<amount>` |
| Seeds (count) | 🫙 | Snapshot line: `seeds: 🫙 x<count>` |
