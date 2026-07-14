# `rammappy` Implementation Details

This document outlines the architectural decisions, design patterns, and specific implementation details for the `rammappy` Python bindings.

## Architecture & FFI Philosophy

The core goal of `rammappy` is to provide a "blazing fast", zero-copy interface to `rammap-core` while exposing an elegant and pythonic API.
To achieve this, we rely heavily on `PyO3`'s capabilities to pass memory boundaries safely without copying data where possible.

### 1. Zero-Copy String Handling (`PyBytes`)

One of the largest overheads in Rust/Python FFI for bioinformatics is string conversion. Python strings are typically UTF-8 decoded, whereas genomic sequences are ASCII or simple bytes. 
- **Decision:** Instead of using `String` or `&str`, we use `Bound<'_, PyBytes>`.
- **Reason:** This allows us to pass raw byte strings (`b"ACGT"`) directly from Python into Rust. `PyBytes` provides zero-copy memory pointers (`as_bytes()`) directly to the underlying Python buffer, eliminating validation and cloning overhead.

### 2. Rayon and the GIL (`map_batch`)

Batch processing is essential for achieving high throughput.
- **Decision:** Implement a `map_batch` method that drops the Global Interpreter Lock (GIL) and distributes mapping tasks across all CPU cores using `rayon`.
- **Reason:** Python code is inherently single-threaded due to the GIL. By explicitly using `py.detach(|| { ... })`, we allow Rust to spawn multiple threads.
- **Implementation Detail:** Passing `Bound<'_, PyBytes>` into threads is normally unsafe because `PyO3` bounds are tied to the GIL. We use a raw pointer approach (`*const u8`) wrapped in a manual `Send + Sync` struct (`RawQuery`). This is safe *only* because the Python byte objects are held in scope for the entire duration of the Rust function call, ensuring the memory is not garbage-collected or modified while the threads are executing.

### 3. Lazy Mappings (`MappingIterator`)

When mapping a sequence, `rammap` may return multiple alignments. 
- **Decision:** Instead of materializing all alignments into a `list` of Python objects immediately, we return a `MappingIterator`.
- **Reason:** Converting `RustMapping` to Python wrapper objects incurs PyO3 allocation overhead. The `MappingIterator` yields `Mapping` objects one at a time on demand. This saves memory and time if the user only cares about the first/primary alignment.

### 4. Stub Generation (`pyo3-stub-gen`)

Static typing is crucial for a modern Python library.
- **Decision:** We use `pyo3-stub-gen` driven by procedural macros (`#[gen_stub_pyclass]`) alongside `ty` (Astral's strict type checker).
- **Reason:** This guarantees that the Python API exactly matches the compiled Rust FFI. We compile a dedicated binary (`stub_gen`) inside the `just install` target to emit `rammappy.pyi` automatically. This allows `mkdocstrings[python]` to statically parse the types and document them via `zensical` without needing to dynamically load the shared library.

### 5. Build System & CI/CD

- **Decision:** Use `maturin` as the PEP 517 build backend without conflicting with `hatch-vcs`.
- **Reason:** `maturin` is the standard for PyO3. Instead of dynamically loading version strings from Git via `setuptools_scm` in `pyproject.toml` (which conflicts with `maturin`), our GitHub Actions (`publish.yml`) directly injects the release tag into `Cargo.toml` before building the wheels. This is simple, deterministic, and plays well with the Rust ecosystem.
