# Non-Interactive Task 03: Output Policy, `print`, and `--stats`

Source:
- `agents/non-interactive-mode.spec.md`
- `agents/non-interactive-mode.plan.md` (Phase 3)

## Goal
Implement text-only output behavior with quiet-by-default execution, on-demand snapshots, and final stats.

## Todo
- [ ] Enforce default quiet runtime (no per-step stdout).
- [ ] Implement `print` command to emit current text snapshot to stdout.
- [ ] Implement `--stats` to emit final text summary to stdout.
- [ ] Ensure all errors go to stderr.
- [ ] Build stats data model containing minimal fields:
  - [ ] `day`
  - [ ] `time` (`HH:MM`)
  - [ ] `location`
  - [ ] `money`
  - [ ] `inventory`
  - [ ] `status` (`ok|error`)

## Acceptance
- [ ] Non-interactive run with no `print` and no `--stats` emits no stdout.
- [ ] `print` outputs human-readable current state.
- [ ] `--stats` outputs final text summary with all required fields represented.
- [ ] stderr/stdout split is consistent and testable.

## Notes
- Output is text only in this iteration (no JSON/quiet flag modes).
- `print` format can be human-readable; stats data fields are the contract.