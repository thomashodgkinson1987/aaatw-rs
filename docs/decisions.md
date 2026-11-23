# Project decisions

This page records the decisions that shape `aaatw-rs`. Keep it up to date when a
decision changes.

## Scope and behaviour

| Decision | Choice | Rationale |
|---|---|---|
| What to replace | The inner `popcapgame1.exe` game only | The outer `AmazingAdventures2.exe` is a DRM wrapper; the goal is the game itself. |
| Launch model | Standalone binary, not a wrapper | Simpler cross-platform design; the original wrapper is not required. |
| Fidelity | 1:1 behavioural clone first | Preserve original gameplay, visuals, timing, assets, and save behaviour. |
| Asset policy | User-supplied original assets | The code is open source; the game assets are proprietary and must not be redistributed. |
| Code licence | MIT | Open-source code with a permissive licence. |
| Rust edition | 2024 | Modern Rust; requires a recent stable toolchain. |
| Repository layout | Cargo workspace with separate crates | Keep core, assets, platform, SDL2, CLI, and tooling separate. |
| Port documentation | In this repository | The LLM wiki is reference material about the original game only. |

## Platform and rendering

| Decision | Choice | Rationale |
|---|---|---|
| Windowing/input/video | SDL2 via the `sdl2` crate | Closest modern equivalent to the original SDL 1.2; cross-platform. |
| Rendering | Software framebuffer + alpha blending | Pixel-perfect 2D blits; avoids a 3D round-trip. |
| Renderer abstraction | `aaatw_core::Renderer` trait | Allows a future `wgpu` backend without changing game logic. |
| Linux release target | `x86_64-unknown-linux-gnu`, dynamic glibc | Simplest reliable SDL2 build for now. |
| Linux aarch64 | Planned, native or cross build | Not yet in CI/release. |
| Static musl | Future goal | The bundled SDL2 CMake build does not cross-compile cleanly to musl yet. |
| Windows release target | `x86_64-pc-windows-msvc` | Toolkit-proven on GitHub-hosted runners; avoids mingw/CMake complexity. |
| Windows local build | `x86_64-pc-windows-gnu` (GCC) | Works locally and produces a standalone GUI-subsystem EXE. |
| macOS targets | `x86_64-apple-darwin` and `aarch64-apple-darwin` | Native builds on `macos-latest`. |

## Asset and game formats

| Decision | Choice | Rationale |
|---|---|---|
| Asset container | Parse `Resource.dll` at runtime | No asset extraction or redistribution required. |
| `std:` URIs | Loose files in the game directory | Matches the original VFS. |
| Other URIs | `Resource.dll` `RAWDATA` resources, uppercased | Matches the original loader. |
| Asset extraction tool | Separate `aaatw-extract` crate | Useful for development and verification; not shipped with the game. |

## Splash behaviour

| Decision | Choice | Rationale |
|---|---|---|
| Splash timing | Hold → fade to black → fade in | Matches the original pre-delay and fade timings. |
| Hold durations | 0.5 s for states 1–3, 1.75 s + 2.5 s for title background | Derived from the original constants and 30 fps capture. |
| Fade duration | 1.25 s | Matches `0x5279e8` and the original capture. |
| Missing distributor image | Show black | `distributor.jpg` is not shipped; the original does the same. |
| `drm/logo.jpg` fallback | Do not use | The original does not use it; it was an incorrect assumption. |

## Workflow

| Decision | Choice | Rationale |
|---|---|---|
| Main branch | `main` | Stable releases. |
| Integration branch | `develop` | Ongoing work. |
| Feature branches | Off `develop`, merged with `--no-ff` | Keeps history explicit. |
| Release merges | `develop` → `main` with `--no-ff` | Stable release history. |
| Versioning | SemVer | Meaningful 0.x releases; 1.0.0 marks API/asset stability. |
| Commit messages | Conventional Commits | Enables git-cliff release notes. |
| Release notes | git-cliff | Generated from Conventional Commits. |
| Changelog | Keep a Changelog | Human-readable release history. |
| Wiki updates | Manual, user-signed on host | The wiki is reference-only and maintained separately. |

## Safety

| Decision | Choice | Rationale |
|---|---|---|
| Game files | Never modify | The game installation is read-only reference material. |
| Assets in repo | Never copy | Proprietary assets must not be redistributed. |
| Analysis artifacts | Outside the repo | Binaries and large dumps stay out of git. |
| Runtime assets | User-supplied | The binary is open source; the assets are not. |
