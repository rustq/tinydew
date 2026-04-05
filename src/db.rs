use rusqlite::{Connection, params};
use std::path::PathBuf;

use crate::state::GameState;

const SCHEMA_VERSION: i32 = 1;

fn db_path() -> PathBuf {
    if let Ok(path) = std::env::var("TINYDEW_DB_PATH") {
        return PathBuf::from(path);
    }

    let base = if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(xdg)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".local").join("share")
    } else {
        PathBuf::from(".")
    };

    base.join("tinydew").join("tinydew.sqlite")
}

pub fn load_or_create() -> GameState {
    let path = db_path();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create data directory");
    }

    let conn = Connection::open(&path).expect("Failed to open database");
    init_db(&conn);

    let payload: Option<String> = conn
        .query_row("SELECT payload FROM game_save WHERE id = 1", [], |row| {
            row.get(0)
        })
        .ok();

    if let Some(json) = payload {
        if let Ok(state) = serde_json::from_str::<GameState>(&json) {
            return state;
        }
    }

    let state = GameState::new();
    save_with_conn(&conn, &state);
    state
}

pub fn save(state: &GameState) {
    let path = db_path();
    let conn = Connection::open(&path).expect("Failed to open database");
    conn.pragma_update(None, "journal_mode", "wal").ok();
    save_with_conn(&conn, state);
}

fn init_db(conn: &Connection) {
    conn.pragma_update(None, "journal_mode", "wal").ok();

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_version (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            version INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS game_save (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            updated_at TEXT NOT NULL,
            payload TEXT NOT NULL
        );",
    )
    .expect("Failed to create tables");

    let version: Option<i32> = conn
        .query_row(
            "SELECT version FROM schema_version WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .ok();

    if version.is_none() {
        conn.execute(
            "INSERT INTO schema_version (id, version) VALUES (1, ?1)",
            params![SCHEMA_VERSION],
        )
        .expect("Failed to insert schema version");
    }
}

fn save_with_conn(conn: &Connection, state: &GameState) {
    let json = serde_json::to_string(state).expect("Failed to serialize state");
    conn.execute(
        "INSERT OR REPLACE INTO game_save (id, updated_at, payload) VALUES (1, datetime('now'), ?1)",
        params![json],
    )
    .expect("Failed to save game state");
}
