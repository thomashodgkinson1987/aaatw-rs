# AGENTS.md — aaatw-rs

This file is the entry point for any agent or contributor working on the Rust port.
Read it first, then [docs/roadmap.md](docs/roadmap.md).

## What this project is

An open-source, cross-platform Rust reimplementation of the inner
`popcapgame1.exe` game from *Amazing Adventures Around the World*
(Steam AppID **3530**, version **1.0.0.5**).

- The original Steam executable `AmazingAdventures2.exe` is a DRM wrapper. It
  extracts the real game as `popcapgame1.exe` and launches it as a child process.
- This project is a **standalone replacement for the inner game only**. It is not a
  wrapper and does not require the original wrapper to run.
- The original game assets are **not included** and must be supplied by the user.
  The code is MIT licensed; the assets remain the property of their rights holders.

## Current status (v0.1.4)

Implemented:

- Cargo workspace with separate core, asset, platform, SDL2, CLI, and extractor crates.
- `Resource.dll` PE resource parser with case-insensitive lookup.
- XUI parser for the game's XML UI format.
- SDL2 800×600 window with software framebuffer and alpha blending.
- Splash sequence and title screen.
- Original splash timing model: hold → fade through black → fade in.
- `aaatw-extract` developer CLI.
- CI for Windows, Linux, and macOS; release workflow for tags.
- Local release artifacts for Windows x86_64 and Linux x86_64.

Not implemented yet:

- Gameplay, levels, puzzles, scoring, missions, journal, map, PDA.
- Audio (BASS replacement).
- Save/config and registry mapping.
- Full XUI rendering (labels, buttons, containers, cursors, fonts).
- Localisation beyond the original resource strings.
- Linux aarch64 release artifacts and static musl builds.

## Start here

1. Read this file.
2. Read [docs/roadmap.md](docs/roadmap.md) for the next tasks.
3. Read [docs/architecture.md](docs/architecture.md) for crate layout and data flow.
4. Read [docs/splash.md](docs/splash.md) for the splash asset/timing findings.
5. Read [docs/original-binary.md](docs/original-binary.md) before doing more
  reverse engineering.
- Read [docs/testing.md](docs/testing.md) for the Wine/Xvfb comparison workflow.
- Read [docs/decisions.md](docs/decisions.md) for project decisions and constraints.
- Reference material:
  - `/workspace/amazing-adventures-wiki` — game reference wiki. It is **not** the
    port's knowledge base; it should only be used for facts about the original game.
  - `/workspace/amazing-adventures-toolkit` — companion patching toolkit.

## Build, test, run

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release -p aaatw-cli
```

Run from the game directory:

```sh
cd "/path/to/Amazing Adventures Around the World"
/path/to/aaatw
```

Useful commands:

```sh
cargo run -p aaatw-extract -- list "/path/to/Resource.dll"
cargo run -p aaatw-extract -- extract-all "/path/to/Resource.dll" assets/
```

## Repository layout

```text
crates/
├── aaatw-core/       # game logic, surfaces, splash state, renderer trait
├── aaatw-assets/     # Resource.dll PE resource parser and XUI parser
├── aaatw-platform/   # platform-neutral input/event types
├── aaatw-sdl2/       # SDL2 window, input, software renderer
├── aaatw-extract/    # asset extraction CLI
└── aaatw-cli/        # the standalone game binary
docs/                 # port documentation and this handoff
.github/workflows/    # CI and release
```

## Analysis artifacts (outside git)

These are not in the repo and may be lost if the container is reset:

- `/workspace/aaatw-analysis/` — Ghidra project, decompiled functions, resource
  list, timing captures, extracted resources.
- `/workspace/aaatw/` — Wine prefix, extracted original `popcapgame1.exe`, original
  test captures, screenshots, logs.
- `/workspace/amazing-adventures-wiki/` — reference wiki.
- `/workspace/amazing-adventures-toolkit/` — companion toolkit.

Ghidra can be re-run headlessly:

```sh
/opt/ghidra/support/analyzeHeadless /workspace/aaatw-analysis/ghidra aaatw \
  -import /workspace/aaatw/popcapgame1.exe
```

Helper scripts are in `/workspace/aaatw-analysis/scripts/`.

## Rules

- **Never modify the game files or assets.** Treat `/workspace` as the live game
  directory and the original files as read-only.
- **Never copy game assets into this repository.** The repo is code and text only.
- The wiki is reference material about the original game. Port documentation goes
  in this repository.
- Use `aaatw-extract` to inspect `Resource.dll`; do not hard-code extracted asset
  bytes into the source.
- Preserve the branch model: `main` stable, `develop` integration, feature branches
  off `develop`, merge with `--no-ff`.
- Follow Conventional Commits and update `CHANGELOG.md` under `Unreleased`.

## Where to start next

See [docs/roadmap.md](docs/roadmap.md). The recommended next milestone is a
**playable vertical slice**: load one hidden-object level, render its background
and objects, handle clicks, and complete the level. That will force the remaining
core systems (XUI rendering, asset formats, input, game state) into shape.