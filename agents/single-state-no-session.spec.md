# Single Persistent Game State (No Session Logic) — Spec

## Goal

Remove MCP session-based gameplay state management and move to a **single persistent game state** model:

- Only one game status exists at runtime.
- All MCP commands operate on that one shared state.
- Game state is automatically saved whenever sleep/income flow completes (day transition).

This makes MCP usage simpler and prevents inventory/world resets caused by starting new sessions.

---

## Motivation

Current session-based behavior can produce confusing outcomes:

- New `startSession` creates a fresh world.
- Inventory/progress appears “lost” when callers unintentionally switch sessions.
- Automation workflows need extra session lifecycle handling.

A single-state model is preferred for this game because gameplay is inherently continuous.

---

## Scope

### In Scope

1. Remove multi-session lifecycle APIs and in-memory session map behavior.
2. Introduce one global active `GameState` in MCP runtime.
3. Auto-save state on sleep/income completion (new day starts).
4. Ensure restart/resume loads last saved state.
5. Keep existing command grammar (`command`, `commandBatch`, `getState`, `getMap`, `getStats`, `print`) working against the single state.

### Out of Scope (for this spec)

- Multiplayer/shared concurrent users.
- Save-slot UI / multiple profiles.
- Cloud sync.

---

## Product Requirements

### PR-1: Single Runtime State

- MCP server must maintain exactly one authoritative `GameState` instance.
- All tools read/write the same instance.
- No per-session state isolation.

### PR-2: Session APIs Compatibility Strategy

Because clients may still call legacy methods, choose one strategy:

#### Option A (Recommended): Soft-compat

- Keep method names `startSession` / `endSession` for compatibility.
- `startSession` returns a constant placeholder id (e.g. `"singleton"`) and does not reset game unless explicit `new_game=true` is requested.
- `endSession` becomes a no-op success.

#### Option B: Hard-break

- Remove `startSession` / `endSession` and require direct command/query methods only.
- Return `NOT_IMPLEMENTED` with migration hint if called.

> Recommended default: Option A for smoother transition.

### PR-3: Auto Save on Sleep/Income Timing

- On day transition flow (`sleep` accepted -> income processing -> `start_new_day`), persist save automatically.
- Save must happen **after** income results are applied and new day state is finalized.
- If save fails, gameplay state remains in memory but MCP response should include a warning.

### PR-4: Load-on-Start Behavior

- At MCP startup, server attempts to load latest save file.
- If save exists and valid: resume from save.
- If missing/invalid: initialize new default game state.

### PR-5: Deterministic Behavior

- Preserve existing weather/day-night/auto-sleep/random spawn logic.
- Removing sessions must not alter core simulation rules.

---

## Technical Design

### 1) State Ownership

Create a singleton container in MCP layer, e.g.:

- `Arc<Mutex<GameState>>` or `Arc<RwLock<GameState>>`
- Initialized once on server startup
- Passed to all handlers

### 2) Remove Session Manager Dependency

Current session manager patterns to deprecate:

- create/get/close session map
- inactivity cleanup
- max concurrent session limits

Replace with:

- global state getter
- optional global metadata (last_save_time, dirty_flag)

### 3) Save Trigger Points

Primary trigger:

- `sleep` command when it completes transition to new day/income finalized.

Optional additional safety triggers (recommended):

- graceful process shutdown
- periodic debounce save (e.g., every N commands or every M minutes if dirty)

### 4) Save File Contract

- Reuse existing `savegame` serialization format where possible.
- Path should remain stable (existing default save path):
  - `dirs::data_local_dir()/shelldew/savegame.json`
  - fallback `./savegame.json`
- Write should be atomic (temp file + rename) to avoid corruption.
- The resolved save path should be exposed in logs (and optionally MCP response metadata) after autosave for debugging.

### 5) MCP Handler Changes

#### `startSession`
- Return singleton reference and current state summary.
- Must not wipe inventory/map by default.

#### `endSession`
- No-op success response.

#### `command` / `commandBatch`
- Ignore incoming `session_id` or validate only that it equals singleton id.
- Operate directly on singleton state.

#### `getState` / `getMap` / `getStats` / `print`
- Read from singleton state.

---

## Data & Error Handling

### Save Failures

If autosave fails:

- Response remains `ok: true` for gameplay command outcome (state already advanced), but include a warning field:
  - `warnings: [{ code: "SAVE_FAILED", message: "..." }]`
- Log full error server-side.

### Corrupt Save on Startup

- Do not crash server.
- Start new game state.
- Emit startup warning log with corruption reason.

---

## Migration Plan

1. Introduce singleton state manager.
2. Wire handlers to singleton path behind feature flag (`single_state_mode`).
3. Add autosave at sleep/new-day completion.
4. Enable compatibility behavior for legacy session methods.
5. Remove old session manager internals after tests pass.

---

## Testing Requirements

### Unit Tests

1. **No reset on repeated startSession**
   - Harvest item, call `startSession` again, inventory remains.

2. **Autosave on sleep**
   - Perform actions, sleep to new day, restart server/runtime, verify state persisted.

3. **endSession no-op**
   - Call endSession, then getState; state unchanged and still accessible.

4. **Corrupt save fallback**
   - Inject invalid save file; startup produces fresh game without crash.

5. **Legacy session_id tolerance**
   - Commands with missing or `singleton` id still work in compat mode.

### Integration Tests

1. Play day 1 -> day 2 with harvest and sleep, then restart MCP runtime; inventory/map/day remain correct.
2. Rainy day forage harvest, sleep, resume; state continuity preserved.
3. commandBatch + sleep produces persisted final state.

---

## Acceptance Criteria

- Using MCP across multiple calls no longer resets world/inventory unless explicit new game action is requested.
- Sleep transitions always persist state automatically.
- Restarting server resumes last saved game.
- Legacy clients calling `startSession` / `endSession` still function (if soft-compat selected).
- Existing gameplay mechanics remain unchanged except session reset behavior.

---

## Open Questions

1. Should there be an explicit `newGame` command that hard-resets state + save?
2. Should autosave trigger only on sleep, or also after major commands?
3. Should singleton state be process-wide lock-only, or include command queue for strict ordering under concurrency?

---

## Suggested MCP Response Additions (Optional)

For observability, add to command responses:

- `state_version` (monotonic increment)
- `autosaved: true|false`
- `save_timestamp`

This helps automation clients verify persistence boundaries.
tamp`

This helps automation clients verify persistence boundaries.
