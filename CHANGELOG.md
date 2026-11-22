# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-22

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