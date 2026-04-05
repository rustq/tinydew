# Single State / No Session Isolation Spec

## Status
Implemented.

## Behavior
- Tinydew runs with one authoritative game state shared by the TUI and CLI.
- Every process launch continues (or creates) the same world via the SQLite save — there is no per-run session isolation or alternate “save slots” in this spec unless added later.

## Persistence (SQLite)

- **Store**: The game uses a **SQLite** database as the only durable save format (no parallel JSON/binary save file).
- **Default path**: Resolve a platform-appropriate user data directory (e.g. XDG data home on Linux, `Application Support` on macOS, `%LOCALAPPDATA%` on Windows), then use a file such as `tinydew/tinydew.sqlite` inside it. The exact subdirectory name may match the app bundle id or `tinydew`.
- **Override**: A single environment variable (e.g. `TINYDEW_DB_PATH`) may override the full path to the database file for tests and portable installs.
- **Lifecycle**:
  - On startup, open the database (create file if missing), run schema migrations if needed, then **load** the authoritative state. If no saved state exists, initialize a new game and persist it.
  - **Auto-save** after CLI `do` command batches, day-transition flows (e.g. sleep), and the same triggers when running the TUI.
- **Durability**: Commits should run inside a **transaction** so a save is atomic. Prefer SQLite **WAL** journal mode where supported. Handle open failures with a clear error path (do not silently discard progress).
- **Schema**: Maintain an explicit **schema version** (table or pragma) and apply incremental migrations when the version changes. Implementation may use normalized tables and/or versioned JSON/TEXT columns for large blobs (maps, crop data), as long as all of it lives in this SQLite file. See **Appendix: example schema** below for a minimal starting point.
- **Build**: Use a Rust SQLite binding such as **`rusqlite`** (with the **bundled** libsqlite feature for reproducible CI/desktop builds unless the project standardizes on system SQLite).

## Persistence Coupling (behavioral)
- State is loaded from the SQLite database on startup when a saved row exists; otherwise a new game is created and written.
- State is autosaved to SQLite after CLI command batches, day-transition flows, and equivalent TUI events.

## Acceptance
- Progress remains across repeated CLI/TUI runs against the same database path.
- No per-session world reset exists in runtime behavior.

---

## Appendix: example schema

Illustrative only — bump `schema_version.version` and migrate when columns or encoding change.

**Option A — one row + JSON document** (fast to ship; maps/crops/inventory live inside `payload`):

```sql
CREATE TABLE schema_version (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  version INTEGER NOT NULL
);

CREATE TABLE game_save (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  updated_at TEXT NOT NULL,  -- ISO-8601 UTC recommended
  payload TEXT NOT NULL      -- e.g. JSON from serde; binary encodings also fine as BLOB
);
```

- On first run: `INSERT INTO schema_version (id, version) VALUES (1, 1);` then insert `game_save` after building initial state.
- On load: read `schema_version.version`, run migrations if `< expected`, then deserialize `game_save.payload`.

**Option B — normalized** — replace `payload` with tables such as `region_tiles(region, x, y, tile_kind, …)`, `crops(…)`, `inventory_counts(item, qty)`, etc., still guarded by the same `schema_version` row.

Either way: **single logical save** (`id = 1`), **WAL + transaction** on write, and **migrations** keyed off `schema_version.version`.
