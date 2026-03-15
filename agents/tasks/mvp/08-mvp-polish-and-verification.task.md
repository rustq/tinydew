# MVP Task 08: MVP Polish & Verification

Source:
- `agents/mvp.spec.md`
- `agents/mvp.plan.md` (Testing + DoD)

## Goal
Finalize MVP with tests, documentation consistency, and green verification gates.

## Todo
- [ ] Add/complete unit tests for movement, farming, growth, trading, forage spawn.
- [ ] Add integration tests for end-to-end gameplay loops.
- [ ] Validate controls match MVP spec (arrows + C/P/W/H/T).
- [ ] Validate UI layout expectation (header + map + message rail).
- [x] Terminal display: CRLF for raw mode; player from state only (no Player in map); path tiles as 🌿, house 🏠 (see spec §4 Terminal display).
- [ ] Run and pass:
  - [ ] `cargo build`
  - [ ] `cargo clippy --all-targets --all-features`
  - [ ] `cargo test`
  - [ ] `cargo fmt --check`
- [ ] Create final MVP completion note/checklist.

## Acceptance
- [ ] All verification commands pass.
- [ ] MVP behavior matches mvp.spec.md with no major gaps.
