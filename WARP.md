# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Repository overview

- Rust binary crate using Bevy 0.16.1
- Entrypoint: `src/main.rs`
- Assets live under `assets/` (UI loads `assets/fonts/MonoLisa.ttf`)
- App: "Nine Lives: Kitten Sudoku" — a 9×9 grid UI; cells cycle through ASCII cat figures when clicked; a "Clear Board" button resets state

## Core commands

### Build, run, and check

```sh
cargo check
cargo run --
cargo build --release
```

### Tests

```sh
# Run all tests (none exist yet, but patterns apply once tests are added)
cargo test
# Run tests matching a substring
cargo test <pattern>
# List tests without running
cargo test -- --list
# Show test output as it happens
cargo test -- --nocapture
```

### Lint and format

```sh
# One-time: ensure tools are installed
rustup component add clippy rustfmt

# Format
cargo fmt --all
# CI-style format check
cargo fmt --all -- --check

# Clippy across all targets and features; fail on warnings
cargo clippy --all-targets --all-features -- -D warnings
```

### Project introspection

```sh
# Dependencies for the current package
cargo tree
# Workspace/package metadata (no deps)
cargo metadata --no-deps --format-version 1
```

## High-level architecture

### State-driven startup

- `AppState` with `Loading` (default) and `Ready`
- Startup system `setup_cat_emojis` inserts the `CatEmojis` resource
- `transition_to_ready` monitors resource availability and sets `NextState<AppState>` to `Ready`
- `setup_grid` runs `OnEnter(AppState::Ready)`, builds the UI tree, and loads the font from `assets/fonts/MonoLisa.ttf`

### ECS data

- Resources:
  - `CatEmojis { emojis: Vec<String> }` — ASCII cat art for digits 1–9
  - `BoardState { cells: [[Option<usize>; 9]; 9] }` — per-cell selected cat index, or `None`
- Components:
  - `Cell { row, col }` on each grid cell button
  - `ClearButton` on the "Clear Board" control

### Systems (Update stage)

- `cell_click_system`: handles `Interaction::Pressed` on grid buttons; cycles the per-cell emoji index in `BoardState`, updates child `Text`, and gives color feedback
- `clear_button_system`: clears `BoardState`, resets all cell `Text` to a placeholder cat, and updates button color on hover/press
- `update_cell_text`: when `BoardState` changes, syncs each cell’s `Text` to match the resource

### UI composition

- `Camera2d` plus a root flex `Node` with column layout and centered content
- Title text and a "Clear Board" button at the top
- 9×9 grid: 9 row `Node`s containing 9 `Button` cells each; every cell has a `Text` child for ASCII cat art
- `AssetServer` loads fonts relative to the `assets/` directory; the code references `"fonts/MonoLisa.ttf"`

## Repo-specific notes

- README vs code mismatch:
  - README states "Bevy 0.14" and "web export support"; `Cargo.toml` uses `bevy = "0.16.1"` and the repo does not include a web runner setup (e.g., Trunk index.html or wasm glue). Desktop builds are supported out of the box; web export will require additional setup.
- Window config: title "Nine Lives: Cat Sudoku", resolution 700×800 (configured in `src/main.rs`)
