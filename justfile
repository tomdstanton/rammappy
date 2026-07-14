# justfile for rammappy

set shell := ["bash", "-uc"]

# Print available commands
default:
    @just --list

# --- Development ---

# Install dependencies, build the extension in development mode, and generate python stubs
install:
    uv venv --allow-existing
    uv pip install -e .
    cargo run --bin stub_gen --no-default-features
    uv run python -c "f = 'python/rammappy/_rammappy/__init__.pyi'; content = open(f).read(); open(f, 'w').write(content.replace('_rammappy.Mapping', 'Mapping'))"


# Build the Rust extension in release mode
build:
    uv venv --allow-existing
    uv pip install -e .

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
    uv run ty check .

# --- CI & Publishing ---

# Run the full CI pipeline locally (format, lint, test)
ci: lint test

# Build production wheels
build-wheels:
    uv run maturin build --release

# Set version in Cargo.toml (useful in CI to sync with git tag)
set-version VERSION:
    uv run python -c "import re; content = open('Cargo.toml').read(); content = re.sub(r'^version\s*=\s*\".*\"', 'version = \"' + '{{VERSION}}'.lstrip('v') + '\"', content, flags=re.MULTILINE); open('Cargo.toml', 'w').write(content)"

# Publish to PyPI
publish: build-wheels
    uv run maturin upload target/wheels/*

# --- Documentation ---

# Build and serve the documentation locally
docs:
    mkdir -p docs
    cp README.md docs/index.md
    cp CONTRIBUTING.md docs/contributing.md
    uv run zensical serve

# Test if documentation can be built without warnings or errors
docs-test:
    mkdir -p docs
    cp README.md docs/index.md
    cp CONTRIBUTING.md docs/contributing.md
    uv run zensical build -s

# Build the documentation into a static site
docs-build:
    mkdir -p docs
    cp README.md docs/index.md
    cp CONTRIBUTING.md docs/contributing.md
    uv run zensical build

# --- Cleanup ---

# Clean build artifacts
clean:
    cargo clean
    rm -rf .venv
    rm -f rammappy.so
    rm -rf rammappy
    rm -rf target/wheels
    rm -rf site
    rm -f docs/index.md docs/contributing.md
    find . -type d -name "__pycache__" -exec rm -rf {} +
