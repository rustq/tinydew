# Development Guide

## Building

### Debug Build
```bash
make build
# or
cargo build
```

### Release Build
```bash
make release
# or
cargo build --release
```

## Running Tests

### All Tests
```bash
make test
# or
cargo test
```

### Verbose Tests
```bash
make test-verbose
# or
cargo test -- --nocapture
```

### Specific Test Module
```bash
cargo test persistence_tests
cargo test farming_tests
cargo test world_tests
cargo test time_tests
```

## Linting

### Run Clippy
```bash
make lint
# or
cargo clippy --all-targets --all-features
```

### Fix Clippy Warnings
```bash
make lint-fix
# or
cargo clippy --fix --allow-dirty
```

## Code Formatting

### Format Code
```bash
make fmt
# or
cargo fmt
```

### Check Formatting
```bash
make fmt-check
# or
cargo fmt --check
```

## Running the Game

```bash
make run
# or
cargo run
```

## Development Workflow

1. **Format** your code before committing:
   ```bash
   make fmt
   ```

2. **Check** for linting issues:
   ```bash
   make lint
   ```

3. **Run tests** to ensure everything works:
   ```bash
   make test
   ```

4. **Build** to verify compilation:
   ```bash
   make build
   ```

## Code Style Guidelines

- Use 4 spaces for indentation
- Maximum line width: 100 characters
- Follow Rust's standard naming conventions
- Add documentation for public APIs
- Write tests for new features
- Use `#[derive(...)]` macros where applicable

## Save System

Game saves are stored in:
- Linux: `~/.local/share/shelldew/saves/`
- macOS: `~/Library/Application Support/shelldew/saves/`
- Windows: `%LOCALAPPDATA%\shelldew\saves\`

Save files:
- `shelldew_save.json` - Main save file
- `autosave.json` - Autosave
- `backup/backup_1.json` - First backup
- `backup/backup_2.json` - Second backup

## Troubleshooting

### Build Errors
- Ensure you have the latest Rust stable toolchain
- Run `cargo clean` and rebuild
- Check for missing dependencies

### Test Failures
- Run tests individually to isolate issues
- Use `cargo test -- --nocapture` for verbose output
- Check test documentation for expected behavior
