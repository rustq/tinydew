# Homebrew Tap Spec

## Status
Draft.

## Goal
Allow users to install TinyDew via Homebrew:
```
brew install rustq/tap/tinydew
```

## Overview
Distribute TinyDew as a Homebrew formula hosted in a separate tap repository (`rustq/homebrew-tap`). Each GitHub release of `rustq/tinydew` publishes prebuilt binaries for macOS (Intel + Apple Silicon) and Linux (x86_64). The tap formula downloads the correct binary for the user's platform.

## Tap Repository

**Repo**: `rustq/homebrew-tap`

This repo holds Homebrew formula files. Minimum contents:

```
homebrew-tap/
  Formula/
    tinydew.rb
```

## Release Artifacts

Each GitHub release (e.g. `v0.1.0`) must attach the following tarballs:

| Artifact | Target | Runner |
|----------|--------|--------|
| `tinydew-v{VERSION}-x86_64-apple-darwin.tar.gz` | macOS Intel | `macos-13` |
| `tinydew-v{VERSION}-aarch64-apple-darwin.tar.gz` | macOS Apple Silicon | `macos-14` |
| `tinydew-v{VERSION}-x86_64-unknown-linux-gnu.tar.gz` | Linux x86_64 | `ubuntu-latest` |

Each tarball contains the single `tinydew` binary at the root.

## Formula

`Formula/tinydew.rb` in the tap repo:

```ruby
class Tinydew < Formula
  desc "A cozy CLI farming game"
  homepage "https://github.com/rustq/tinydew"
  version "{VERSION}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/rustq/tinydew/releases/download/v#{version}/tinydew-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "{SHA256_MACOS_ARM}"
    else
      url "https://github.com/rustq/tinydew/releases/download/v#{version}/tinydew-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "{SHA256_MACOS_X86}"
    end
  end

  on_linux do
    url "https://github.com/rustq/tinydew/releases/download/v#{version}/tinydew-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "{SHA256_LINUX}"
  end

  def install
    bin.install "tinydew"
  end

  test do
    assert_match "tinydew", shell_output("#{bin}/tinydew -V")
  end
end
```

Placeholders (`{VERSION}`, `{SHA256_*}`) are filled by the release workflow.

## GitHub Actions — Release Workflow

**File**: `.github/workflows/release.yml` in `rustq/tinydew`.

**Trigger**: Push a tag matching `v*` (e.g. `v0.1.0`).

### Jobs

#### 1. `build` (matrix)

Runs on each target platform in parallel.

| Matrix entry | Runner | Target | Artifact name |
|---|---|---|---|
| `x86_64-apple-darwin` | `macos-13` | `x86_64-apple-darwin` | `tinydew-v{VERSION}-x86_64-apple-darwin.tar.gz` |
| `aarch64-apple-darwin` | `macos-14` | `aarch64-apple-darwin` | `tinydew-v{VERSION}-aarch64-apple-darwin.tar.gz` |
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` | `x86_64-unknown-linux-gnu` | `tinydew-v{VERSION}-x86_64-unknown-linux-gnu.tar.gz` |

Steps:
1. Checkout `rustq/tinydew`.
2. Install stable Rust toolchain.
3. `cargo build --release --target ${{ matrix.target }}`.
4. `tar -czf tinydew-v{VERSION}-{target}.tar.gz -C target/{target}/release tinydew`.
5. Upload tarball as workflow artifact.

#### 2. `release` (depends on `build`)

Steps:
1. Download all build artifacts.
2. Create GitHub release for the tag with all tarballs attached (`gh release create`).

#### 3. `update-tap` (depends on `release`)

Steps:
1. Compute SHA256 for each tarball:
   ```bash
   SHA_MACOS_ARM=$(shasum -a 256 tinydew-v*-aarch64-apple-darwin.tar.gz | awk '{print $1}')
   SHA_MACOS_X86=$(shasum -a 256 tinydew-v*-x86_64-apple-darwin.tar.gz | awk '{print $1}')
   SHA_LINUX=$(shasum -a 256 tinydew-v*-x86_64-unknown-linux-gnu.tar.gz | awk '{print $1}')
   ```
2. Checkout `rustq/homebrew-tap`.
3. Render `Formula/tinydew.rb` with the computed version and SHA256 values.
4. Commit and push to `rustq/homebrew-tap` main branch.

Requires a `TAP_GITHUB_TOKEN` secret with push access to `rustq/homebrew-tap`.

## Release Process

1. Update `version` in `Cargo.toml`.
2. Commit: `Bump version to X.Y.Z`.
3. Tag and push:
   ```bash
   git tag vX.Y.Z
   git push origin vX.Y.Z
   ```
4. The `release.yml` workflow builds binaries, creates the GitHub release, and updates the tap formula automatically.
5. Users can then run:
   ```bash
   brew install rustq/tap/tinydew
   ```

## User Experience

```bash
# First install
brew install rustq/tap/tinydew

# Upgrade
brew upgrade tinydew

# Uninstall
brew uninstall tinydew

# Play
tinydew status
tinydew do move right
```

## Notes
- The tap repo must be named exactly `homebrew-tap` so Homebrew resolves `rustq/tap` correctly.
- Cross-compilation is avoided — each target builds natively on its matching runner.
- `rusqlite` uses the `bundled` feature, so no system SQLite dependency is needed for the formula.
- If Linux ARM support is needed later, add an `aarch64-unknown-linux-gnu` matrix entry with a cross-compilation step or ARM runner.
