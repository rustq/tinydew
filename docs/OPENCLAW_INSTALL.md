# Tinydew Installation (OpenClaw one-doc guide)

This document is a single, copy-paste setup guide for getting **Tinydew** running in an OpenClaw environment.

---

## 1) Prerequisites

Install required tools:

- `git`
- `rustup` (Rust toolchain)
- `cargo`

Check:

```bash
git --version
rustc --version
cargo --version
```

If Rust is missing:

```bash
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
```

---

## 2) Clone Tinydew

```bash
git clone https://github.com/rustq/tinydew.git
cd tinydew
```

(If already cloned, just `cd tinydew`.)

---

## 3) Build

Standard build:

```bash
cargo build --release
```

Interactive build (TUI):

```bash
cargo build --release --features interactive
```

---

## 4) Run Modes

### A) MCP mode (for OpenClaw automation)

Run without interactive feature:

```bash
cargo run --quiet
```

- In non-TTY context, Tinydew falls back to MCP stdio server.
- OpenClaw can send JSON requests to stdin (`startSession`, `getState`, `command`, etc.).
- Recommended chat behavior: for every user message during game sessions, call `print` and include the latest Tinydew UI snapshot in the assistant reply.

### B) Interactive mode (guest/player keyboard control)

```bash
cargo run --quiet --features interactive
```

Controls:

- Arrow keys: move
- Space: guest greet (in guest mode)
- Esc: quit

---

## 5) Save File Location

Tinydew save path:

```text
~/.local/share/tinydew/savegame.json
```

Reset save (fresh Day 1):

```bash
rm -f ~/.local/share/tinydew/savegame.json
```

---

## 6) OpenClaw usage note

For OpenClaw automation, prefer MCP mode and send tool-style commands:

- `startSession`
- `getState`
- `command` / `commandBatch`
- `print` for text UI snapshots

Common MCP command strings:
- `move:up|down|left|right`
- `clear`, `plant:<crop>`, `water`, `harvest`
- `fish` or `fish:up|down|left|right`
- `buy:<item>[:<qty>]`, `sell:<item>[:<qty>]`, `print`, `save`, `load

This gives deterministic, scriptable game control.

### Suggested response format for chat

When replying in game flow:
1. One short status line (what happened).
2. A fenced code block with the latest `print` snapshot UI.
3. Optional one-line next prompt (e.g., "move where?").

Example:

```text
Done — moved to Square.

=== Tinydew Day 3 09:10 ☀️ ===
Location: Square
...
> Entered Square.
```
