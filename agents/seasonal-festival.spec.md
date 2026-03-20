# Seasonal Festival Spec

## Status
Implemented.

## Season Model
- Season is tracked in game state (current runtime centered on Spring behavior).

## Butterfly Festival (Spring Day 28)
- Weather forced Sunny.
- Default bottom line:
  `Today is Butterfly Festival, enjoy it!`
- Wonder tile (`🦋`) appears at Square `(2,2)`.
- Wonder is non-walkable for both player and guest.
- Attempting to step onto Wonder shows:
  `That is so beautiful. Let human enjoy it together in interactive mode.`

## Greeting Text
- Normal days: time/weather-aware greeting messages.
- Festival guest greet (Space in interactive guest mode):
  `✨ Happy Butterfly Festival!`
- Interactive renderer allows festival guest greeting to display as override text.
