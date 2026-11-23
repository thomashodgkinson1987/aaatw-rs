# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Tag-driven release workflow with artifacts for Windows MSVC, Linux x86_64/aarch64
  glibc, and macOS x86_64/aarch64.
- git-cliff configuration for generating release notes from Conventional Commits.
- CI and release badges.

### Changed

- CI now runs on `ubuntu-latest`, `windows-latest`, and `macos-latest`, and uses
  current GitHub Actions versions (`actions/checkout@v5`, `actions/upload-artifact@v6`,
  `Swatinem/rust-cache@v2`).
- macOS builds no longer use the retired `macos-14`/`macos-13` runner images.
- Splash screens now use the original dwell durations and crossfade transitions.

### Fixed

- `std:` URIs now resolve as loose files in the game directory, matching the
  original VFS. Non-`std:` URIs resolve through `Resource.dll`.
- Removed the incorrect `drm/logo.jpg` fallback for the missing `distributor.jpg`.

## [0.1.0] - 2026-11-23

### Added

- Initial Cargo workspace with Rust 2024 edition.
- `aaatw-core`: colour, surface, renderer trait, asset provider trait, application
  state machine, and splash/title screens.
- `aaatw-assets`: read-only PE resource parser for `Resource.dll`, plus an XUI
  parser for the game's XML UI format.
- `aaatw-platform`: platform-neutral game event and key types.
- `aaatw-sdl2`: SDL2 window, input, and software renderer backend.
- `aaatw-extract`: command-line tool to list and extract `Resource.dll` resources.
- `aaatw-cli`: standalone game binary that loads the original assets and renders the
  splash sequence and title screen.
- GitHub Actions CI configuration for Windows, Linux, and macOS.
- Keep a Changelog, contributing guide, MIT license, and initial documentation.

[Unreleased]: https://github.com/thomashodgkinson1987/aaatw-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/thomashodgkinson1987/aaatw-rs/releases/tag/v0.1.0