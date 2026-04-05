# Initial Map Spec

## Regions (initial world graph)
- **Farm** ↔ **EastPath**
- **EastPath** ↔ **Square**
- **EastPath** ↔ **SouthRiver**

## Coordinate system
- **(x, y)** is **(column, row)**, **0-indexed**.
- Dimensions listed as **width x height**.

## Farm
- **Dimensions**: 8 x 8
- **Boundaries**: full boundary ring (all edges are `Boundary` / non-walkable)
- **Fixed tiles**
  - `House` at **(2,2)** (non-walkable)
  - Transition tile to EastPath: `PathEast` at **(7,5)**
- **Connection**
  - **Farm -> EastPath**
    - Trigger: stepping onto `PathEast` at **(7,5)**
    - Spawn in EastPath at **(1,2)**, facing **Right**
  - **EastPath -> Farm**
    - Trigger: stepping onto `PathFarm` (see EastPath)
    - Spawn in Farm at **(6,5)**, facing **Left**

## EastPath
- **Dimensions**: 11 x 4
- **Boundaries**: top row is mostly boundary; bottom row is mostly boundary; right edge is boundary.
- **Fixed tiles**
  - Transition tile to Farm: `PathFarm` at **(0,2)**
  - Transition tile to Square: `PathSquare` at **(5,0)**
  - Transition tile to SouthRiver: `PathSouthRiver` at **(2,3)**
  - `Mushroom` at **(9,2)** (non-walkable blocker; harvestable forage)
- **Connections**
  - **EastPath -> Farm**
    - Trigger: stepping onto `PathFarm` at **(0,2)**
    - Spawn in Farm at **(6,5)**, facing **Left**
  - **Farm -> EastPath**
    - Trigger: stepping onto Farm `PathEast`
    - Spawn in EastPath at **(1,2)**, facing **Right**
  - **EastPath -> Square**
    - Trigger: stepping onto `PathSquare` at **(5,0)**
    - Spawn in Square at **(4,3)**, facing **Up**
  - **Square -> EastPath**
    - Trigger: stepping onto Square `PathSquare` (see Square)
    - Spawn in EastPath at **(5,1)**, facing **Down**
  - **EastPath -> SouthRiver**
    - Trigger: stepping onto `PathSouthRiver` at **(2,3)**
    - Spawn in SouthRiver at **(2,1)**, facing **Down**
  - **SouthRiver -> EastPath**
    - Trigger: stepping onto SouthRiver `PathSouthRiverGate` (see SouthRiver)
    - Spawn in EastPath at **(2,2)**, facing **Up**

## Square
- **Dimensions**: 9 x 5
- **Boundaries**: full boundary ring (all edges are `Boundary` / non-walkable)
- **Fixed tiles**
  - `Fountain` at **(4,2)** (non-walkable)
  - A pre-placed `Flower` at **(1,1)** (non-walkable; decorative spawn, not a planted crop)
  - Transition tile back to EastPath: `PathSquare` at **(4,4)**
- **Connection**
  - **EastPath -> Square**
    - Trigger: stepping onto EastPath `PathSquare` at **(5,0)**
    - Spawn in Square at **(4,3)**, facing **Up**
  - **Square -> EastPath**
    - Trigger: stepping onto `PathSquare` at **(4,4)**
    - Spawn in EastPath at **(5,1)**, facing **Down**

## SouthRiver
- **Dimensions**: 13 x 4
- **Fixed tiles**
  - Transition tile back to EastPath: `PathSouthRiverGate` at **(2,0)**
  - River occupies rows **y=2..3** (inclusive) across all columns **x=0..12**
    - `River` / `RiverBubble` tiles are **non-walkable** and **fishable**
- **Connection**
  - **EastPath -> SouthRiver**
    - Trigger: stepping onto EastPath `PathSouthRiver` at **(2,3)**
    - Spawn in SouthRiver at **(2,1)**, facing **Down**
  - **SouthRiver -> EastPath**
    - Trigger: stepping onto `PathSouthRiverGate` at **(2,0)**
    - Spawn in EastPath at **(2,2)**, facing **Up**
