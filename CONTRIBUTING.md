# Contributing to detaxine-charts

Thank you for your interest in contributing! This document outlines how to get started, the conventions we follow, and how to submit your work.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Adding a New Chart](#adding-a-new-chart)
- [Code Conventions](#code-conventions)
- [Running Tests](#running-tests)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Reporting Issues](#reporting-issues)

## Getting Started

Prerequisites:

- [Rust](https://rustup.rs/) (stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Trunk](https://trunkrs.dev/)
- Firefox (for headless WASM tests)

Clone and build:

    git clone https://github.com/elonaire/detaxine-charts.git
    cd detaxine-charts
    cargo build

Run the dashboard example locally:

    cd src/core
    trunk serve --example dashboard

Then open http://localhost:8080.

## Project Structure

    detaxine-charts/
    ├── src/core/                  # the published library crate
    │   ├── src/
    │   │   ├── lib.rs             # crate root and doc examples
    │   │   ├── charts/
    │   │   │   ├── mod.rs         # chart module declarations
    │   │   │   ├── bar_chart/
    │   │   │   ├── pie_chart/
    │   │   │   ├── doughnut_chart/
    │   │   │   ├── line_chart/
    │   │   │   └── candlestick_chart/
    │   │   └── utils/
    │   │       └── hooks/
    │   │           └── use_chart_data.rs
    │   ├── examples/
    │   │   └── dashboard.rs       # dashboard example (deployed to GitHub Pages)
    │   ├── index.html             # Trunk entry point
    │   └── Cargo.toml
    ├── Cargo.toml                 # workspace root
    ├── CONTRIBUTING.md
    └── README.md

## Development Workflow

1. Fork the repository and create a branch from `main`:

        git checkout -b feat/my-new-chart

2. Make your changes inside `src/core/`
3. Run tests before pushing (see [Running Tests](#running-tests))
4. Open a pull request against `main`

Branch naming conventions:

| Prefix | Use for |
|---|---|
| `feat/` | New charts or features |
| `fix/` | Bug fixes |
| `docs/` | Documentation only |
| `chore/` | CI, dependencies, tooling |
| `refactor/` | Code changes with no behaviour change |

## Adding a New Chart

Every chart in this library follows the same pattern. When adding a new chart:

1. Create the module at `src/core/src/charts/my_chart/mod.rs`

2. Follow the established pattern:
   - Accept `Signal<Vec<T>>` for reactive data
   - Store geometry in `StoredValue` after each draw for hit testing
   - Use two separate `Effect`s if you have both state and redraw concerns (see candlestick chart)
   - Return geometry from the draw function — never recompute on hover
   - No `unwrap()` or `expect()` — use `let Some(...) else { return }`
   - Use `context.reset_transform()` before `context.scale()` on every redraw

3. Add a feature flag in `src/core/Cargo.toml`:

        [features]
        MyChart = []

4. Register the module in `src/core/src/charts/mod.rs`:

        #[cfg(feature = "MyChart")]
        pub mod my_chart;

5. Re-export from `src/core/src/lib.rs`:

        #[cfg(feature = "MyChart")]
        pub use charts::my_chart::{MyChart, MyChartConfig};

6. Add a doc example in `lib.rs` following the existing chart examples
7. Add to the dashboard in `charts-dashboard/src/main.rs`
8. Write a wasm test for the draw function

## Code Conventions

- **No unwraps** - every fallible operation uses `let Some(...) else { return }` or `.ok()?`
- **Draw functions are pure** - they take data and config as arguments, return geometry, and have no side effects
- **`get_untracked()`** inside redraw closures - the `Effect` already tracks the signal, don't track it again inside the draw call
- **Slanted x-axis labels** - use `context.save()`, `translate`, `rotate(-PI / 4.0)`, `restore()` for all x-axis labels
- **`Memo` for derived data** - use `Memo::new` for anything derived from a signal that's used in the view (e.g. legend metadata)
- **Semver** — this crate mirrors Leptos major and minor versions. Patch is incremented independently

## Running Tests

Run the WASM test suite from inside the library crate:

    cd src/core
    wasm-pack test --firefox --headless

Run doc tests:

    cd src/core
    cargo test --doc

## Submitting a Pull Request

- Keep PRs focused — one feature or fix per PR
- Make sure all tests pass before opening a PR
- Update doc examples in `lib.rs` if you changed a public API
- Update the dashboard in `charts-dashboard/src/main.rs` if you added a new chart
- Add an entry to `CHANGELOG.md` describing your change under `Unreleased`

## Reporting Issues

Please open an issue on [GitHub](https://github.com/elonaire/detaxine-charts/issues) with:

- A minimal reproduction if it's a bug
- The Leptos version you are using
- The browser you are testing in (charts render in WASM so browser matters)

## License

By contributing, you agree that your contributions will be dual licensed under MIT and Apache 2.0, consistent with the rest of the project.
