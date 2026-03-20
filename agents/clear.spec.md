# Clear Action Spec

## Status
Implemented.

## Rules
- `clear` targets adjacent/default tile according to current action targeting.
- Clear converts valid clearable tile to grass/soil cleanup outcome per tile type.
- Cannot clear blocked/building tiles.
- Square region forbids clear actions globally.

## Messaging
- Invalid/blocked targets return rejection messages (e.g. cannot clear here/target).
