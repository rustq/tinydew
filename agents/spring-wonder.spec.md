# Spring Day 28 Butterfly Festival Spec

## Goal
Define Spring Day 28 as a special **Butterfly Festival** day with consistent world behavior and player-facing messaging.

## Feature Summary
- Festival day: **Spring Day 28**
- Festival tile: `Wonder` (`🦋`) at **Square `(x=2, y=2)`**
- Festival weather: **forced Sunny**
- Festival bottom/status text (UI + MCP snapshot):
  - `Today is Butterfly Festival, enjoy it!`

---

## Rules

### 1) Festival Trigger
On day-start update (`start_new_day` flow), when:
- `season == "Spring"`
- `day == 28`

the game must activate Butterfly Festival behavior.

### 2) Wonder Spawn Rule
During festival activation:
- Place `TileType::Wonder` at `square_map[2][2]`.

### 3) Weather Rule
During weather roll on festival day:
- Force `weather = Sunny`
- Mark `weather_day = day`
- Skip normal random weather selection for that day.

### 4) UI / Snapshot Message Rule
On festival day, bottom message text is overridden globally to:
- `Today is Butterfly Festival, enjoy it!`

Applies to:
- Interactive UI bottom message area
- MCP `print` snapshot message line

### 5) Wonder Interaction Rule
Wonder is non-walkable and special:
- Player cannot step onto it.
- Guest also cannot step onto it.
- Attempting to move onto it shows:
  - `That is so beautiful. Let human enjoy it together in interactive mode.`

---

## Data Model
- `TileType::Wonder` must remain available and render as `🦋`.
- Wonder remains non-walkable.

---

## Acceptance Criteria
1. On Spring Day 28, `square_map[2][2]` is `Wonder (🦋)`.
2. Spring Day 28 weather is always **Sunny**.
3. Bottom message on Spring Day 28 is always:
   - `Today is Butterfly Festival, enjoy it!`
4. Wonder blocks movement for both player and guest, keeping them on current tile.
5. Attempting to step onto Wonder shows the Wonder interaction message.
6. Behavior is consistent in interactive mode and MCP `print` snapshot.

---

## Suggested Tests
- `test_wonder_spawns_on_spring_day_28_at_square_2_2`
- `test_wonder_tile_renders_butterfly_emoji`
- `test_wonder_tile_is_not_walkable`
- `test_wonder_message_on_player_attempted_step`
- `test_wonder_message_on_guest_attempted_step`
- `test_spring_day_28_weather_forced_sunny`
- `test_spring_day_28_snapshot_message_forced_butterfly_festival`
