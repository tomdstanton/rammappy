# 🔗🐍🧬 `rammappy`
* A Python interface to [rammap](https://github.com/jwanglab/rammap), the Rust implementation of minimap2*

[![Release](https://img.shields.io/github/v/release/tomdstanton/rammappy)](https://img.shields.io/github/v/release/tomdstanton/rammappy)
[![PyPI](https://img.shields.io/pypi/v/rammappy.svg?logo=PyPI)](https://pypi.org/project/rammappy)
[![Ruff](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json)](https://github.com/astral-sh/ruff)
[![ty](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ty/main/assets/badge/v0.json)](https://github.com/astral-sh/ty)
[![Stars](https://img.shields.io/github/stars/tomdstanton/rammappy.svg?style=social&maxAge=3600&label=Star)](https://github.com/tomdstanton/rammappy/stargazers)

---

## Introduction

`rammappy` is designed with a focus on speed, lazy evaluation, and parallel processing. It is built using [PyO3](https://github.com/PyO3/pyo3) and the [Maturin](https://github.com/PyO3/maturin) build backend.

## Features

- **Zero-Copy Byte FFI:** Uses Python `bytes` (`&[u8]`) directly instead of parsing expensive UTF-8 strings.
- **Multithreaded Parallel Alignment:** Drops the Python GIL during batch processing to map thousands of reads concurrently using Rust's Rayon ecosystem.
- **Lazy Iteration:** Emits a `MappingIterator` instead of a static list to preserve memory usage during massive mapping evaluations.
- **Exposed Extensibility Sketchers:** Direct access to `MinimizerSketcher`, `SyncmerSketcher`, and `RandstrobeSketcher`.

## Example

```python
import rammappy

# Define target sequences and queries
target = b"ACGT" * 100
queries = [
    (b"query1", b"ACGT" * 20),
    (b"query2", b"CGTA" * 20),
]

# Instantiate Aligner
aligner = rammappy.Aligner([(b"contig1", target)], preset=rammappy.Preset.Sr)

# Perform zero-copy parallel alignment
batch_results = aligner.map_batch(queries)

# Lazy-evaluate the iterator to pull hits
for i, mappings in enumerate(batch_results):
    first_hit = next(iter(mappings), None)
    if first_hit:
        print(f"Query {i+1} mapped to {first_hit.target_name.decode()} at {first_hit.target_start}")
```

## Core Design Philosophy
The overriding goal for this project was to establish **extremely performant, zero-copy FFI bindings** linking the Rust core API to Python. A focus was applied to maintain "bare metal Rust" speeds while providing a clean and "lazy" Python API.

---

### Zero-Copy Evaluation Using `bytes`
Python strings (`str`) perform computationally expensive UTF-8 allocations and validation mechanisms. `rammappy` universally prefers Python byte-strings (`bytes` in Python, mapped to `&[u8]` in Rust). 

Data entering the alignment algorithm uses `Bound<'py, PyBytes>`, mapping directly to contiguous blocks of C-memory storing the sequences. Retrieving genomic strings mapping fields (e.g., CIGAR/MD/CS strings) exposes byte payloads directly without additional UTF-8 reallocations during FFI crossings.

### Parallel Scaling and the Python GIL
Scaling genomic query alignments parallel on multi-core systems mandates threading. However, the presence of the Python Global Interpreter Lock (GIL) poses problems, as normal PyO3 structures retain a lock on the main Python thread.

**The Solution (`map_batch` with Rayon):**
1. We collect batches of targets mapped to pointers and lengths.
2. We wrap these representations in a custom struct `RawQuery { name_ptr, seq_ptr, ... }`.
3. We implement `unsafe impl Send for RawQuery` and `unsafe impl Sync for RawQuery`. This allows pointer transmission to threads across thread boundaries.
4. Using `py.detach(|| ...)` (formerly `py.allow_threads` prior to PyO3 0.21.0+), we entirely drop the Python GIL.
5. Within this detached closure, the `rayon` crate (`par_iter`) iterates the `RawQuery` objects and maps across all CPU cores.
6. `unsafe { std::slice::from_raw_parts }` safely rebuilds the byte vectors in the isolated thread spaces. It is memory-safe because the parent function's stack continues to live over the closure, ensuring the memory bound to `PyBytes` remains allocated.

### Lazy Materialization
Rather than computing alignment lists as heavy `Vec<Mapping>` aggregates and immediately converting every hit to a Python-native Object (costing heavy FFI time), `rammappy` returns a `MappingIterator`.

A `MappingIterator` acts as an opaque handle holding the vector of internal `RustMapping` entries. Only when a user invokes `next(iterator)` is the memory read and a single Python `Mapping` object initialized and surfaced over the FFI boundary.

### `Sketcher` Extension Points
`rammappy` exposes direct hooks into three separate Sketcher implementations (`MinimizerSketcher`, `SyncmerSketcher`, and `RandstrobeSketcher`). They wrap `rammap-core` struct equivalents.
Their `.sketch()` methods immediately map the resulting minimizers onto a heavily minimal Python `Minimizer` struct containing `x` and `y` integer positions available as straightforward Python class properties.

## FFI Object Hierarchy
* **`Aligner`:** Core orchestrator parsing targets and executing `map` or `map_batch`.
* **`Preset`:** Python enumeration delegating pre-compiled aligner configurations to Rust (e.g. `Preset.MapOnt`).
* **`MappingIterator`:** Yield generator.
* **`Mapping`:** Read-only struct acting as an accessor to individual `rammap-core` hit fields.
* **`MinimizerSketcher`, `SyncmerSketcher`, `RandstrobeSketcher`:** Core interfaces evaluating seeds.
* **`Minimizer`:** Payload returned from Sketchers.
# rammappy Implementation

## 1. Rust-Python Bridge (PyO3)

We use `PyO3` as the foundational bridge. The library exposes a series of structures:
- `Aligner`: The primary engine.
- `MappingIterator` and `Mapping`: For lazy evaluation of results.
- `Preset`: Enum for defining genome configuration.
- Sketchers (`MinimizerSketcher`, `SyncmerSketcher`, `RandstrobeSketcher`): For extracting sequences.

### Design Choices
1. **Zero-Copy / Byte-Strings Preference**: We prioritize passing `&Bound<'_, PyBytes>` directly from Python to the Rust core to avoid string validation and utf-8 decoding overhead. This provides blazing fast performance.
2. **Lazy Evaluation**: The `Aligner` returns a `MappingIterator`. Instead of instantiating all mapped results into Python objects inside a `Vec<Mapping>` (which costs memory and time), we yield `Mapping`s one-by-one by tracking the underlying `std::vec::IntoIter<RustMapping>`.
3. **Internal `rammap-core` Memory Model**: Since we assume no control over `rammap-core`'s internal memory management for now, we instantiate configurations dynamically and map objects lazily to prevent large overhead.

## 2. Zero-Copy Threaded Batch Alignment (`rayon`)

To utilize threading effectively across multiple sequence queries, the standard `map` function executes sequentially on Python's primary thread. Python possesses a Global Interpreter Lock (GIL) preventing true multithreading of native Python logic.

To circumvent this and process large batches of queries efficiently:
1. We introduced the `map_batch` method inside `Aligner`.
2. `map_batch` receives a `Vec<(Bound<'_, PyBytes>, Bound<'_, PyBytes>)>` (Name and Sequence for each query).
3. We extract **Raw Pointers** (`*const u8`) alongside lengths to represent the raw memory array of each `PyBytes` block.
4. We enforce `unsafe impl Send` and `unsafe impl Sync` on these raw pointer representations (`RawQuery`), which tells Rust they are safe to share across threads.
5. The GIL is released via `py.detach(|| { ... })`.
6. Inside the GIL-free closure, `rayon::prelude::*` is used (`par_iter().map()`) to fan out the alignments onto all available CPU cores.
7. Resulting `MappingIterator` vectors are collected and returned to Python upon GIL re-acquisition.

This achieves genuine, zero-copy, highly parallel throughput in pure Rust, circumventing Python's GIL while interacting directly with Python-owned byte strings!

## 3. Python Stubs Generation (`pyo3-stub-gen`)

To allow autogenerated documentation (e.g., via `pdoc` or Sphinx) and type hinting for modern Python IDEs:
1. `pyo3-stub-gen` is configured.
2. We replaced standard PyO3 macros with `#[gen_stub_pyclass]`, `#[gen_stub_pymethods]`, and `#[gen_stub_pyclass_enum]`.
3. A `define_stub_info_gatherer!(stub_info);` registry macro is instantiated at the module level.
4. An integrated `cargo test` unit test (`fn generate_stubs()`) builds and outputs an `__init__.pyi` (Python Interface file) detailing all signatures, arguments, and return types.

## 4. Test Framework & Build

Instead of relying on the legacy `test.py` wrapper, we have structured the repository to match modern Python standards:
- The `tests/test_rammappy.py` file uses `pytest` for declarative testing logic.
- We utilize `uv` to create a virtual environment (`uv pip install -e .`) avoiding complex `.so` copying which often shadows system installations.
- **Gotcha Noted**: We discovered that executing tests with a locally lingering `rammappy/` directory (created during stub-gen or earlier experiments) or a manual `rammappy.so` file in the root directory causes `AttributeError` failures! Python prioritizes local matching directories over the virtual environment `site-packages`. 
- To fix this, `just test` cleans the local path before invoking pytest, preventing name clashing.
