# Non-Interactive Task 01: CLI Args & Mode Switch

Source:
- `agents/non-interactive-mode.spec.md`
- `agents/non-interactive-mode.plan.md` (Phase 1 + Phase 4)

## Goal
Add batch-mode CLI entrypoint and route execution between interactive TUI and non-interactive runner.

## Todo
- [ ] Add CLI parsing with `clap`.
- [ ] Define/parse supported flags:
  - [ ] `--batch`, `-b`
  - [ ] `--command <cmd>`, `-c <cmd>` (repeatable)
  - [ ] `--stats`
- [ ] Enforce batch-mode rule: at least one `--command` required when `--batch` is set.
- [ ] Add `main.rs` mode switch:
  - [ ] `--batch` -> non-interactive execution path
  - [ ] default -> existing TUI path
- [ ] Ensure unsupported non-scope flags are not exposed (`--script`, `--output`, state init flags).

## Acceptance
- [ ] `shelldew --batch -c "move:right"` enters non-interactive path.
- [ ] `shelldew --batch` fails with clear validation error and exit code `1`.
- [ ] `shelldew` (no `--batch`) still starts TUI behavior unchanged.

## Notes
- Keep flag name as `--batch` (decision locked).
- No JSON/quiet output flags in this iteration.