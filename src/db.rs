use std::path::PathBuf;

use rusqlite::{Connection, params};

use crate::state::GameState;

const SCHEMA_VERSION: i32 = 1;

pub fn db_path() -> PathBuf {
    if let Ok(path) = std::env::var("TINYDEW_DB_PATH") {
        return PathBuf::from(path);
    }

    // XDG data home on Linux, Application Support on macOS, %LOCALAPPDATA% on Windows
    let data_dir = if cfg!(target_os = "macos") {
        dirs_fallback("Library/Application Support")
    } else if cfg!(target_os = "windows") {
        std::env::var("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| dirs_fallback(".local/share"))
    } else {
        std::env::var("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| dirs_fallback(".local/share"))
    };

    data_dir.join("tinydew").join("tinydew.sqlite")
}

fn dirs_fallback(subdir: &str) -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(subdir)
}

pub fn open_db() -> Connection {
    let path = db_path();

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }

    let conn = Connection::open(&path).expect("Failed to open database");

    // Enable WAL mode
    conn.execute_batch("PRAGMA journal_mode=WAL;")
        .expect("Failed to set WAL mode");

    // Initialize schema
    init_schema(&conn);

    conn
}

fn init_schema(conn: &Connection) {
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

    // Check/insert schema version
    let version: Option<i32> = conn
        .query_row(
            "SELECT version FROM schema_version WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .ok();

    match version {
        None => {
            conn.execute(
                "INSERT INTO schema_version (id, version) VALUES (1, ?1)",
                params![SCHEMA_VERSION],
            )
            .expect("Failed to insert schema version");
        }
        Some(v) if v < SCHEMA_VERSION => {
            // Run migrations here if needed
            conn.execute(
                "UPDATE schema_version SET version = ?1 WHERE id = 1",
                params![SCHEMA_VERSION],
            )
            .expect("Failed to update schema version");
        }
        _ => {}
    }
}

pub fn load_state(conn: &Connection) -> Option<GameState> {
    conn.query_row(
        "SELECT payload FROM game_save WHERE id = 1",
        [],
        |row| {
            let json: String = row.get(0)?;
            Ok(serde_json::from_str(&json).expect("Failed to deserialize game state"))
        },
    )
    .ok()
}

pub fn save_state(conn: &Connection, state: &GameState) {
    let json = serde_json::to_string(state).expect("Failed to serialize game state");
    let now = chrono_now();

    conn.execute(
        "INSERT OR REPLACE INTO game_save (id, updated_at, payload) VALUES (1, ?1, ?2)",
        params![now, json],
    )
    .expect("Failed to save game state");
}

fn chrono_now() -> String {
    // Simple UTC timestamp without chrono dependency
    use std::time::SystemTime;
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}
