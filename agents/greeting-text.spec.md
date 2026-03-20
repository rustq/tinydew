# Greeting Text Spec

## Status
Implemented.

## Sources
- General runtime message field (`state.message`).
- Guest greeting via interactive Space key.
- Snapshot display message adapter in MCP `print`.

## Behavior
- Time/weather-aware greetings are used for normal days.
- Festival default line on Spring Day 28:
  `Today is Butterfly Festival, enjoy it!`
- Guest Space greeting on Spring Day 28:
  `✨ Happy Butterfly Festival!`
- Interactive rendering allows festival guest greeting override to be shown.
