# justfile for rammappy

set shell := ["bash", "-uc"]

# Print available commands
default:
    @just --list

# --- Development ---

# Install dependencies and build the extension in development mode
install:
    uv pip install -e .

# Build the Rust extension in release mode and copy it to the root
build:
    cargo build --release
    cp target/release/librammappy.dylib rammappy.so

# Run the test suite
test: install
    uv run pytest tests/

# --- Linting & Formatting ---

# Run all formatters (Rust + Python)
fmt: fmt-rust fmt-python

# Run all linters (Rust + Python)
lint: lint-rust lint-python

# Format Rust code
fmt-rust:
    cargo fmt

# Format Python code
fmt-python:
    uv run black .

# Lint Rust code
lint-rust:
    cargo clippy --all-targets --all-features -- -D warnings

# Lint Python code
lint-python:
    uv run ruff check .

# --- CI & Publishing ---

# Run the full CI pipeline locally (format, lint, test)
ci: lint test

# Build production wheels
build-wheels:
    uv run maturin build --release

# Publish to PyPI
publish: build-wheels
    uv run maturin upload target/wheels/*

# --- Cleanup ---

# Clean build artifacts
clean:
    cargo clean
    rm -rf .venv
    rm -f rammappy.so
    rm -rf rammappy
    rm -rf target/wheels
    find . -type d -name "__pycache__" -exec rm -rf {} +
