# 🌱 Tinydew

A cozy farming game with both interactive play and MCP automation support.

## What is Tinydew?

Tinydew is a lightweight Rust game where you can:
- move between regions (Farm, EastPath, Square, SouthRiver)
- farm, fish, trade, and progress days
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
