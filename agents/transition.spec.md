# Region Transition Spec

## Status
Implemented.

## Regions
- Farm
- EastPath
- Square
- SouthRiver

## Gate Behavior
- Farm <-> EastPath via path tiles.
- EastPath <-> Square via path-square gate pairing.
- EastPath <-> SouthRiver via dedicated gate pairing.
- Transition updates location + entity coordinates + direction/message.
- Both player and guest have region transition handlers.
