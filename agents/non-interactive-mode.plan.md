# Non-Interactive Mode Implementation Plan

## Overview

Implement command-driven non-interactive mode for Shelldew CLI using `--batch`, with text output only.

## Scope (Final)

Included:
- `--batch` mode switch (name retained)
- Repeatable `--command` input
- `--stats` final text summary
- `print` command for on-demand state snapshots
- `buy` and `sell` commands for market actions

Excluded:
- `--script <file>`
- `--output` (`quiet|json|text` selector removed)
- State initialization flags (`--day`, `--hour`, `--money`, `--season`, `--location`, `--seed`, `--inventory`)
- `trade` toggle command in non-interactive mode

Behavior decisions:
- Runtime is quiet by default (no per-step output).
- Output appears only for `print` command and `--stats`.
- `plant` requires explicit crop parameter (`plant:<crop>`).

## Phased Implementation

### Phase 1: CLI Argument Parsing
- Add `clap` crate for argument parsing
- Add/keep flags: `--batch`, `--command/-c`, `--stats`
- Enforce in batch mode: at least one `--command`
- Remove/avoid `--output`, `--script`, and state-init args from batch scope

### Phase 2: Command Parser + Executor
- Create runner module (e.g., `src/non_interactive.rs`)
- Parse commands:
  - `move:<direction>`
  - `clear` (adjacent auto-target)
  - `plant:<crop>` (required crop; adjacent auto-target)
  - `water` (adjacent auto-target)
  - `harvest` (adjacent auto-target)
  - `buy:<item>[:<qty>]`
  - `sell:<item>[:<qty>]`
  - `sleep`
  - `print`
  - `quit`
- Reject `trade` in batch mode with clear error
- Execute sequentially; stop on first error

### Phase 3: Output + Stats
- Default command execution emits no stdout
- `print` emits current state snapshot (text)
- `--stats` emits final summary (text)
- Implement stats data model with minimal schema fields:
  - `day`, `time`, `location`, `money`, `inventory`, `status`

### Phase 4: Main Integration
- Update `src/main.rs` mode switch:
  - `--batch` -> non-interactive runner
  - otherwise -> existing TUI
- Keep interactive behavior unchanged

## Expected File Touch Points

| File | Changes |
|------|---------|
| `Cargo.toml` | Add `clap` dependency |
| `src/main.rs` | CLI parse + mode switch |
| `src/cli.rs` (new) | Batch arg definitions + validation |
| `src/non_interactive.rs` (new) | Command parse/execute loop |
| `src/output.rs` (optional) | Shared text rendering helpers |
| `src/lib.rs` | Export new modules if needed |

## Testing Strategy

### Unit Tests
- Command parser happy/invalid paths
- `plant` requires crop
- `buy/sell` quantity parsing and validation
- `trade` rejected in batch mode

### Integration Tests
- `--batch -c "move:right"` succeeds (exit 0)
- Multiple `-c` commands run in order
- `print` produces stdout snapshot
- `--stats` produces final stdout summary
- Invalid command or arg exits 1

## Exit Codes

- `0` on successful run completion
- `1` on any validation/execution error
- No other exit code classes in this iteration

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Command behavior diverges from TUI | Reuse existing action/state methods |
| Ambiguous buy/sell item naming | Define strict item enum and validate early |
| Quiet-by-default surprises users | Document `print` and `--stats` clearly in README |

## Rollout Checklist

- [ ] Add CLI flags: `--batch`, `--command`, `--stats`
- [ ] Implement non-interactive parser + executor
- [ ] Add `buy`/`sell` batch commands
- [ ] Enforce `plant:<crop>` explicit syntax
- [ ] Reject `trade` in batch mode
- [ ] Implement quiet-by-default output policy
- [ ] Add `print` and `--stats` text outputs
- [ ] Add unit + integration tests
- [ ] Update README examples
- [ ] Verify `cargo fmt`, `cargo clippy`, and `cargo test` pass
