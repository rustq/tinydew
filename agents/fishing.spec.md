# Fishing Spec

## Status
Implemented.

## Behavior
- Fishing: `tinydew do fish <dir>` (see `cli.spec.md`).
- Default fishing supports adjacent auto-targeting near river.
- Fishing advances time and can yield common/rare fish or no bite.
- River bubble tile lifecycle is supported and reset by sleep/day transition.
- Fish inventory persists in the SQLite game database (see `single-state-no-session.spec.md`).
- Fish can be sold via `tinydew do sell …` with value tracking per item type.
