# 🌱 Tinydew

A cozy farming game with both interactive play and MCP automation support.

## What is Tinydew?

Tinydew is a lightweight Rust game where you can:
- move between regions (Farm, EastPath, Square, SouthRiver)
- farm, fish, trade, and progress days
- play piano with guests at the North Square
- run fully in MCP mode for automation agents like OpenClaw

---

## Developer Quick Start

### 1) Prerequisites

- `git`
- `rustup` / `rustc`
- `cargo`

Check:

```bash
git --version
rustc --version
cargo --version
```

### 2) Clone

```bash
git clone https://github.com/rustq/tinydew.git
cd tinydew
```

### 3) Build

Release build:

```bash
cargo build --release
```

Interactive-enabled build:

```bash
cargo build --release --features interactive
```

### 4) Run

MCP mode (automation / non-TTY):

```bash
cargo run --quiet
```

Interactive mode:

```bash
cargo run --quiet --features interactive
```

### 5) Test

```bash
cargo test
```

---

## Piano Keyboard

Tinydew features a piano at the North Square where guests can play music.

**Keyboard Mapping:**
- Q, W, E, R, T, Y, U, I, O, P → C Major scale (C4–E5) using Salamander Grand Piano samples

See [`PianoKeyboard.md`](./PianoKeyboard.md) for full details.

---

## Save Data

Default save location:

```text
~/.local/share/tinydew/savegame.json
```

Reset save:

```bash
rm -f ~/.local/share/tinydew/savegame.json
```

---

## OpenClaw Installation Guide

For full OpenClaw setup and usage notes, see:

- [`OPENCLAW_INSTALL.md`](./OPENCLAW_INSTALL.md)

This includes:
- OpenClaw-oriented install flow
- MCP usage notes
- suggested chat/UI snapshot behavior

---

## Save Data

Default save location:

```text
~/.local/share/tinydew/savegame.json
```

Reset save:

```bash
rm -f ~/.local/share/tinydew/savegame.json
```
