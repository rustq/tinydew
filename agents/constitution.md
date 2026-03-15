# Agents Constitution

## Purpose

This constitution defines how autonomous agents operate inside the `shelldew` workspace. It explains why the `agents/` directory exists, the expectations for every agent contribution, and the guardrails that keep work product reliable.

## Guiding Principles

1. **Clarity first** – Every agent must leave behind artifacts that explain intent and observable outcomes. Vague plans and half-complete work are not acceptable.
2. **Continuity of ownership** – Tasks are carried through to verification. Passing unfinished work forward requires explicit status notes and open questions.
3. **Safety of the codebase** – No destructive git operations, no suppression of errors, and no secrets committed. Prevention beats cleanup.
4. **Reproducibility** – Steps to reproduce bugs or verify fixes must be documented or scripted. If it cannot be reproduced, it is not done.
5. **Respect for precedent** – Reuse existing patterns before inventing new ones unless there is a strong, documented reason to diverge.

## Directory Layout

- `agents/constitution.md` – This document. Update it when adding new norms.
- `agents/<agent-name>/` – Workspace for a specific agent or initiative. Each subdirectory must include a README outlining scope, current tasks, and any automation scripts.

Additional files (logs, scratch pads, data exports) belong inside the relevant agent subdirectory. Temporary or experimental data should be clearly labeled and ignored via `.gitignore` if it does not belong in version control.

## Operating Protocols

### Task Intake

1. Record every incoming request as a todo with concrete exit criteria.
2. Clarify ambiguities by inspecting the repository and history before asking stakeholders.
3. Begin execution only after the plan is captured (even if lightweight) so that others can trace decisions.

### Execution Discipline

- Instrument work with meaningful commits or change notes; avoid lumping unrelated changes together.
- Prefer deterministic tooling (formatters, linters, tests) over manual assurance. When tooling is unavailable, describe manual verification steps explicitly.
- Keep partial progress in feature branches; never leave `main` in a broken state.

### Verification

Before marking any todo complete:

1. Run `cargo fmt` and `cargo clippy` whenever Rust source files change.
2. Execute the relevant test suites (`cargo test`, integration runners, or custom scripts) and capture failures in issue trackers if they pre-date the change.
3. Summarize what was verified, including command names and noteworthy output.

### Knowledge Transfer

- Document new decisions in `agents/<agent-name>/README.md` or architecture notes.
- Reference related issues or pull requests so future agents can reconstruct context quickly.
- When pausing work, leave breadcrumbs: remaining todos, blockers, and pointers to logs or dashboards.

## Governance and Updates

- Amend this constitution when workflows evolve. Every modification must explain why the change helps agents operate more effectively.
- Major disputes (tooling, branching strategy, escalation paths) should be resolved in consultation with maintainers and codified here once consensus is reached.

Maintainers review this document quarterly to ensure it reflects actual practice. Agents are empowered—and expected—to propose improvements whenever gaps appear.

---

## Verification Reference

All verification commands must pass before marking any task complete.

### Standard Verification Commands

```bash
# Build the project
cargo build

# Run Clippy linter
cargo clippy --all-targets --all-features

# Run all tests
cargo test

# Check code formatting
cargo fmt --check
```

### Makefile Shortcuts

The project includes a Makefile for common operations:

```bash
make build        # cargo build
make test         # cargo test
make lint         # cargo clippy --all-targets --all-features
make lint-fix     # cargo clippy --fix --allow-dirty
make fmt          # cargo fmt
make fmt-check    # cargo fmt --check
make run          # cargo run
```

### CI Pipeline

The project runs CI checks on all pushes to main/develop and pull requests. The CI pipeline includes:
- Format check (cargo fmt --check)
- Clippy lints (cargo clippy --all-targets --all-features -D warnings)
- Tests (cargo test --all-features)
- Release build (cargo build --release)
- Nightly build (cargo test --all-features + cargo clippy --all-targets --all-features -D warnings)

### Verification Checklist

Before marking any todo complete, ensure:
1. `cargo build` succeeds without errors
2. `cargo clippy --all-targets --all-features` passes with zero warnings
3. `cargo test` passes all tests
4. `cargo fmt --check` passes (code is formatted)
5. New features have corresponding tests
6. Documentation is updated if APIs change
