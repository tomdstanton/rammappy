# Contributing to `rammappy`

First off, thank you for considering contributing to `rammappy`! It's people like you that make open-source bioinformatics
tools better for the entire community.

Whether you are fixing a bug, improving alignment performance, adding a new feature, or simply correcting
a typo in the documentation, your help is welcome.

## Table of Contents
1. [Where to Start](#where-to-start)
2. [Reporting Bugs & Requesting Features](#reporting-bugs-requesting-features)
3. [Local Development Setup](#local-development-setup)
4. [Testing and Benchmarks](#testing-and-benchmarks)
5. [Building the Documentation](#building-the-documentation)
6. [Submitting a Pull Request](#submitting-a-pull-request)

***

## Where to Start

If you are looking for a way to contribute, check out our [GitHub Issues](https://github.com/tomdstanton/rammappy/issues).
Look for issues tagged with `good first issue` or `help wanted`.

If you plan to work on a major feature or a significant architectural change to the Rust bindings, please open an issue
first to discuss it with the maintainers so we can ensure it aligns with the project's roadmap.

## Reporting Bugs & Requesting Features

We use GitHub Issues to track bugs and feature requests. 

* **Bugs:** Please provide a clear description of the issue, the expected behavior, and ideally, a minimal reproducible
example (including a small sample FASTA file if applicable).

* **Features:** Explain the feature, why it would be useful for sequence mapping workflows, and how it should work.

## Local Development Setup

`rammappy` is a hybrid project relying on Python and a Rust backend via PyO3. We use `uv` as our blazing-fast package and
project manager.

### Prerequisites

1. Install [Rust](https://www.rust-lang.org/tools/install) (`cargo`).
2. Install [uv](https://docs.astral.sh/uv/getting-started/installation/).

### Getting Started

1. Fork the repository on GitHub and clone your fork locally:

```console
git clone https://github.com/tomdstanton/rammappy.git
cd rammappy
```

Create a new branch for your feature or bugfix:

```console
git checkout -b feature/my-new-feature
```

Use our `just` task runner to install the project in editable mode, compile the Rust extensions, build Python stubs, and set up your virtual environment:

```console
just install
```

(Note: Whenever you make changes to the `.rs` files in the `src/` directory, you will need to re-run the `just install`
command to recompile the PyO3 bindings and regenerate the stubs).

## Testing and Benchmarks

We use pytest for our test suite. It is crucial that any new features are covered by unit tests and do not break the
existing alignment logic.

To run the standard test suite, use the `just` task runner:

```console
just test
```

## Building the Documentation

Our documentation is built using Zensical. The site is automatically deployed via GitHub Actions, but you should
preview your changes locally before submitting a PR.

Make your changes to the `README.md` or `CONTRIBUTING.md` files in the root directory (they will be copied to `docs/` automatically during the build), or update the Python docstrings.

Start the local live-reloading server:

```console
just docs
```

Open http://localhost:8000 in your browser.

(Note: Do not commit the `site/`, `docs/index.md`, or `docs/contributing.md` files to version control as they are generated dynamically).

## Submitting a Pull Request
When you are ready to share your code:

Ensure your code is formatted correctly and passes all tests.

Push your branch to your GitHub fork:

```console
git push origin feature/my-new-feature
```

Open a Pull Request against the main branch of the upstream rammappy repository.

Fill out the provided PR template, describing your changes and linking to any relevant issues.

A maintainer will review your code. We may request some changes, but don't worry—we are here to help!

***

**Thank you for helping make `rammappy` faster and more robust you superstar you!** ✨