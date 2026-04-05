# Seasonal Festival Spec

## Status
Implemented.

## Season Model
- Season is tracked in game state (current runtime centered on Spring behavior).

## Butterfly Festival (Spring Day 28)
- Weather forced Sunny.
- Default bottom line:
  `Today is Butterfly Festival, enjoy it!`
- Wonder tile (`🦋`) appears at Square `(2,2)` on Day 28.
- After sleeping/day-transition beyond the festival day, Wonder is reset back to Grass at `(2,2)`.
- Wonder is non-walkable for the player.
- Attempting to step onto Wonder shows:
  `That is so beautiful. Let's enjoy it together in the game.`

## Greeting Text
- Normal days: time/weather-aware greeting messages.
