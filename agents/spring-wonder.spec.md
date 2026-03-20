# Spring Day 28 Butterfly Festival Spec

## Status
Implemented.

## Festival Rules
- Trigger day: Spring Day 28.
- Weather: forced Sunny.
- Festival line (default bottom text):
  `Today is Butterfly Festival, enjoy it!`

## Wonder Tile
- Wonder (`🦋`) appears at Square `(2,2)` on festival day.
- Wonder is non-walkable.
- Player stepping attempt onto Wonder is blocked with:
  `That is so beautiful. Let human enjoy it together in interactive mode.`
- Guest stepping attempt onto Wonder is also blocked with same message.

## Guest Festival Greeting
- In interactive guest mode, pressing Space on festival day shows:
  `✨ Happy Butterfly Festival!`
- This greeting is allowed to appear instead of the default festival bottom line.
