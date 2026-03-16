# Non-Interactive Task 04: Validation, Tests, and Docs

Source:
- `agents/non-interactive-mode.spec.md`
- `agents/non-interactive-mode.plan.md` (Testing + Rollout)

## Goal
Lock correctness with tests and publish usage docs for new non-interactive behavior.

## Todo
- [ ] Add unit tests for parser/validation:
  - [ ] command tokenization + dispatch
  - [ ] `plant:<crop>` required
  - [ ] qty validation for buy/sell
  - [ ] `trade` rejection in batch mode
- [ ] Add integration tests for CLI behavior:
  - [ ] `--batch -c "move:right"` success path
  - [ ] multiple `-c` command ordering
  - [ ] `print` writes stdout snapshot
  - [ ] `--stats` writes final summary
  - [ ] invalid command exits `1`
- [ ] Update README with command-only non-interactive examples.
- [ ] Verify and record green checks:
  - [ ] `cargo fmt`
  - [ ] `cargo clippy --all-targets --all-features`
  - [ ] `cargo test`

## Acceptance
- [ ] All tests pass consistently.
- [ ] README reflects final scope decisions (no script/state-init/output-mode flags).
- [ ] Verification commands are green.

## Definition of Done
- [ ] Tasks 01-03 complete
- [ ] Tests and README complete
- [ ] Tooling checks green (`fmt`, `clippy`, `test`)