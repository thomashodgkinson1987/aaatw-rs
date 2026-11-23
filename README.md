# aaatw-rs

[![CI](https://github.com/thomashodgkinson1987/aaatw-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/thomashodgkinson1987/aaatw-rs/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/thomashodgkinson1987/aaatw-rs)](https://github.com/thomashodgkinson1987/aaatw-rs/releases/latest)
[![License: MIT](https://img.shields.io/github/license/thomashodgkinson1987/aaatw-rs)](LICENSE)
[![Rust edition 2024](https://img.shields.io/badge/rust-edition%202024-orange)](https://doc.rust-lang.org/edition-guide/2024/index.html)
[![Platforms](https://img.shields.io/badge/platform-windows%20%7C%20linux%20%7C%20macOS-blue)](#targets)

Open-source, cross-platform Rust reimplementation of the inner game binary from
*Amazing Adventures Around the World* (Steam AppID **3530**, version **1.0.0.5**).

The original Steam release ships a DRM wrapper (`AmazingAdventures2.exe`) which
extracts and launches the real game as `popcapgame1.exe`. This project reimplements
that inner game as a standalone Rust binary. It is **not** a wrapper and does not
require the original wrapper to run.

> **Assets are not included.** You must supply your own legally obtained copy of
> `Resource.dll`, `bass.dll`, `config.txt`, and the other game files. The code in
> this repository is MIT licensed; the game assets remain the property of their
> respective rights holders.

## Status

`v0.1.0` is the project scaffold and first vertical slice:

- Cargo workspace with separate core, asset, platform, SDL2, CLI, and extractor crates.
- PE resource parser for `Resource.dll`.
- XUI parser for the game's XML UI format.
- SDL2 software-rendered 800x600 window with nearest-neighbour scaling.
- Splash-screen sequence and title screen.

It is not yet a playable game. See [CHANGELOG.md](CHANGELOG.md) for the current
state and [docs/](docs/) for design notes.

## Requirements

- Rust 1.85+ (Rust 2024 edition)
- SDL2 (the `sdl2` crate can build it from source with the `bundled` feature)
- A copy of the original game assets in the working directory

## Build

```sh
cargo build --release -p aaatw-cli
```

The binary will be at `target/release/aaatw` (or `aaatw.exe` on Windows).

To run it from the game directory:

```sh
cd "/path/to/Amazing Adventures Around the World"
/path/to/aaatw
```

The game looks for `Resource.dll` in the executable's directory and the current
working directory. You can override the asset directory with `--data-dir`.

Linux release builds target x86_64 and aarch64 glibc. Static musl builds are a
future goal; the bundled SDL2 CMake build does not yet cross-compile cleanly to musl
in this setup.

## Asset extraction tool

`aaatw-extract` is a developer tool for inspecting and extracting resources from
`Resource.dll`:

```sh
cargo run -p aaatw-extract -- list "/path/to/Resource.dll"
cargo run -p aaatw-extract -- extract "/path/to/Resource.dll" SPLASHSCREENS.XUI out.xui
cargo run -p aaatw-extract -- extract-all "/path/to/Resource.dll" assets/
```

## Targets

Current release targets:

- Windows x86_64 (`x86_64-pc-windows-msvc`)
- Linux x86_64 (`x86_64-unknown-linux-gnu`, glibc)
- Linux aarch64 (`aarch64-unknown-linux-gnu`, glibc)
- macOS x86_64 (`x86_64-apple-darwin`)
- macOS aarch64 (`aarch64-apple-darwin`)

Static musl Linux builds are a future goal.

## Releases

Releases are produced by the [Release workflow](.github/workflows/release.yml) when a
`v*` tag is pushed. Artifacts are named:

```text
aaatw-<version>-<target>.zip     # Windows
aaatw-<version>-<target>.tar.gz  # Linux and macOS
```

Each archive contains the `aaatw` binary, `README.md`, and `LICENSE`. A
`SHA256SUMS` file is attached to the GitHub Release.

## Repository layout

```text
crates/
├── aaatw-core/       # game logic, surfaces, XUI-independent application state
├── aaatw-assets/     # Resource.dll PE resource parser and XUI parser
├── aaatw-platform/   # platform-neutral event and platform traits
├── aaatw-sdl2/       # SDL2 backend
├── aaatw-extract/    # asset extraction CLI
└── aaatw-cli/        # the game binary
```

## License

MIT. See [LICENSE](LICENSE).