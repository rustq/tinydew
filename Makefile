.PHONY: build release test lint fmt clean check install run

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

test-verbose:
	cargo test -- --nocapture

lint:
	cargo clippy --all-targets --all-features

lint-fix:
	cargo clippy --fix --allow-dirty

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

check:
	cargo check --all-targets --all-features

build-release:
	cargo build --release

clean:
	cargo clean

all: fmt-check lint test build

install:
	cargo install --path .

run:
	cargo run

help:
	@echo "Available targets:"
	@echo "  build          - Debug build"
	@echo "  release        - Release build"
	@echo "  test           - Run tests"
	@echo "  lint           - Run clippy lints"
	@echo "  lint-fix       - Fix clippy warnings"
	@echo "  fmt            - Format code"
	@echo "  fmt-check      - Check code formatting"
	@echo "  check          - Check for errors"
	@echo "  clean          - Clean build artifacts"
	@echo "  all            - Run fmt, lint, test, and build"
