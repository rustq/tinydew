# Single Persistent Game State (No Session Logic) — Implementation Plan

## References

- Spec: `agents/single-state-no-session.spec.md`
- Existing MCP docs/specs: `agents/mcp.spec.md`, `agents/mcp.plan.md`

---

## Objective

Implement a singleton game-state architecture for MCP runtime so gameplay is continuous across calls, with autosave on sleep/income day transitions.

---

## Delivery Strategy

Use a phased rollout with compatibility-first behavior:

1. Introduce singleton state manager and route reads/writes through it.
2. Keep legacy `startSession`/`endSession` methods as compatibility wrappers.
3. Add autosave trigger on sleep/day transition completion.
4. Remove session-manager internals after validation.

---

## Phase 0 — Baseline & Guardrails

### Tasks

1. Capture current behavior with tests for:
   - repeated `startSession` resets state
   - `endSession` closes access
2. Add migration TODO markers in MCP modules where session assumptions exist.
3. Define compatibility mode constant (`SINGLETON_SESSION_ID = "singleton"`).

### Files likely touched

- `src/mcp/handler.rs`
- `src/mcp/session.rs`
- `src/mcp/tools.rs`
- `tests/mcp_*.rs` (or existing unit tests in module files)

### Exit criteria

- Baseline tests are in place and failing once singleton behavior starts (expected during migration).

---

## Phase 1 — Singleton State Container

### Tasks

1. Create a global MCP runtime state container:
   - `Arc<Mutex<GameState>>` (or `RwLock` if needed)
2. Initialize once on MCP server startup.
3. Load persisted save at startup:
   - success -> use loaded state
   - failure/missing -> create default state and log warning
4. Expose accessor API for handlers.

### Files likely touched

- `src/mcp/mod.rs`
- `src/mcp/server.rs`
- `src/mcp/session.rs` (repurpose or replace with singleton manager)
- `src/savegame.rs` (if loader helper changes are needed)

### Exit criteria

- A single state instance is created and readable by handlers.
- Server startup resumes prior save when present.

---

## Phase 2 — Handler Routing Migration (No Per-Session State)

### Tasks

1. Refactor all MCP handlers to use singleton state directly:
   - `command`
   - `commandBatch`
   - `getState`
   - `getMap`
   - `getStats`
   - `getWorldTime`
2. Deprecate session lookup failures (`SESSION_NOT_FOUND`) in singleton path.
3. Ignore or soft-validate incoming `session_id`.

### Compatibility behavior

- Accept absent `session_id`.
- Accept `session_id == "singleton"`.
- Optionally tolerate any string and treat as singleton during transition.

### Files likely touched

- `src/mcp/handler.rs`
- `src/mcp/schema.rs`
- `src/mcp/errors.rs`

### Exit criteria

- All command/query methods operate against one shared game state.
- No in-memory per-session map required for gameplay.

---

## Phase 3 — Legacy Method Compatibility

### Tasks

1. `startSession` soft-compat:
   - returns singleton id
   - does not reset game by default
   - optional `new_game=true` support only if explicitly implemented
2. `endSession` soft-compat:
   - no-op success response
3. Update response payloads/docs for compatibility semantics.

### Files likely touched

- `src/mcp/handler.rs`
- `src/mcp/tools.rs`
- `agents/mcp.spec.md` (if updating public contract)

### Exit criteria

- Legacy clients continue functioning without losing state.

---

## Phase 4 — Autosave on Sleep/Income Transition

### Tasks

1. Detect command outcomes that trigger day transition completion.
2. Persist state immediately after transition finalizes.
3. Implement atomic save write (temp + rename) if not already guaranteed.
4. Add warning propagation in response when save fails:
   - gameplay remains `ok`
   - include warnings metadata
5. Keep autosave destination aligned with existing save resolver:
   - primary: `dirs::data_local_dir()/shelldew/savegame.json`
   - fallback: `./savegame.json`
   - log resolved path after each autosave event

### Detection guidance

- Prefer explicit state-change signal over string matching messages.
- If needed, add a small flag in game state transition path (e.g., `just_started_new_day`).

### Files likely touched

- `src/state.rs` (optional transition signal)
- `src/mcp/handler.rs`
- `src/savegame.rs`
- `src/mcp/schema.rs`

### Exit criteria

- Sleeping into next day consistently triggers autosave.
- Restart resumes post-sleep state.

---

## Phase 5 — Session Manager Decommission

### Tasks

1. Remove unused session lifecycle code:
   - create/get/close/cleanup/session limits
2. Remove stale error paths no longer applicable.
3. Keep only singleton-specific runtime state metadata:
   - optional `dirty` bit
   - optional `last_save_time`

### Files likely touched

- `src/mcp/session.rs` (delete or repurpose)
- `src/mcp/errors.rs`
- MCP tests/docs

### Exit criteria

- No dead session-map code remains.
- Build/test passes cleanly without unused session internals.

---

## Phase 6 — Tests, Verification, and Docs

### Required tests

1. **No reset on repeated startSession**
2. **endSession no-op**
3. **Autosave after sleep transition**
4. **Restart resumes saved inventory/day/map**
5. **Corrupt save fallback to fresh state**
6. **commandBatch + sleep persists final state**
7. **Legacy session_id compatibility**

### Manual verification checklist

1. Start MCP runtime.
2. Harvest item.
3. Call `startSession` again.
4. Confirm inventory unchanged.
5. Sleep to next day.
6. Restart runtime.
7. Confirm resumed state includes post-sleep data.

### Docs updates

- `agents/mcp.spec.md` (compat behavior notes)
- `agents/tasks/mcp/README.md` (usage changes)
- Any CLI/help text that mentions session lifecycle assumptions

### Exit criteria

- All tests green.
- Docs reflect singleton + autosave model.

---

## Suggested Work Breakdown (Commit-sized)

1. **commit A**: Introduce singleton state bootstrap + load-on-start
2. **commit B**: Route handlers to singleton state
3. **commit C**: Soft-compat start/end session semantics
4. **commit D**: Autosave on sleep/day transition + save warnings
5. **commit E**: Remove old session manager internals
6. **commit F**: Tests + docs update

---

## Risks & Mitigations

### Risk 1: Hidden dependencies on session_id
- **Mitigation**: Temporary compatibility shim accepts legacy ids.

### Risk 2: Save corruption or partial write
- **Mitigation**: Atomic save strategy + robust startup fallback.

### Risk 3: Autosave trigger misses edge path
- **Mitigation**: Add explicit transition flag and assert in tests.

### Risk 4: Concurrent command races
- **Mitigation**: Guard all state mutations with one lock and serialized command handling.

---

## Definition of Done

- Game has one authoritative MCP state.
- Repeated `startSession` no longer resets progress.
- `endSession` does not destroy active game.
- Sleep/income day transition autosaves reliably.
- Restart resumes latest persisted state.
- Tests and docs updated for singleton architecture.
