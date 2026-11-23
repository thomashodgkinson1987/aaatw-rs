# Roadmap

This is the working roadmap for `aaatw-rs`. It is intentionally high-level; update
it when a milestone is completed or the scope changes.

## Guiding target

A standalone, cross-platform Rust binary that is a **1:1 behavioural clone** of the
original `popcapgame1.exe` (v1.0.0.5), using the original assets at runtime.

“1:1” means gameplay, visuals, audio, timing, save/config behaviour, and UI layout
match the original. Platform-specific windowing and rendering differences live
behind the platform abstraction layer.

## Milestones

### M0 — Project scaffold ✅

- Cargo workspace, crates, CI, branch model, changelog, contributing guide.
- Build/test/release workflow.
- Local Windows and Linux release artifacts.

### M1 — Asset and XUI foundations ✅

- `Resource.dll` PE resource parser.
- XUI parser.
- `aaatw-extract` CLI.
- Asset name lookup and extraction tests.

### M2 — Splash and title slice ✅

- SDL2 800×600 window.
- Splash sequence and title screen.
- Original hold/fade-through-black timing.
- Correct `std:` loose-file asset resolution.
- SpinTop splash and title background match the original assets.

### M3 — Full XUI rendering

Goal: render the game's actual UI screens, not just splash images.

- Parse all XUI documents, including external entities (`&fonts;`, `&textures;`,
  `&audio;`, `&mainmenu;`, `&map;`, `&game;`, `&pda;`, `&journal;`, `&dialogs;`).
- Implement:
  - `<image>` with texture, position, size, flags.
  - `<container>` traversal.
  - `<label>` with fonts and colours.
  - `<button>` / `aa:allbutton` / `aa:scenebutton`.
  - `<frame>` with RGBA fills.
  - `<slider>`, `<checkbox>`, `<radiobutton>`, `<editbox>`.
  - `<cursor>` and cursor shapes.
- Implement font loading and text rendering.
- Implement texture atlases / `.MSE` parsing.
- Acceptance: the main menu and options screens render and respond to input.

### M4 — One playable hidden-object level

Goal: a vertical slice through the core game loop.

- Load one `SCENE_*.XML` / `SPOT_*.STD` level.
- Render background, foreground, overlay, and hidden objects.
- Handle mouse hover, click, found/not-found feedback.
- Implement hint, timer, score, and level-complete flow.
- Acceptance: one level can be completed from start to finish.

### M5 — All game modes

Goal: implement every original game mode.

- Hidden-object / EyeSpy.
- Spot the difference.
- Jigsaw.
- Word search.
- Tile game.
- Tile rotation game.
- Match-one / Simon-says.
- Place game.
- Final puzzle.
- Journal, map, PDA, high scores, missions.

Acceptance: every mode is playable with original assets and rules.

### M6 — Platform and persistence parity

- Audio: replace BASS with a cross-platform backend (`rodio`/`symphonia` or similar).
- Save/config:
  - Windows: real registry keys under `HKCU\Software\PopCap\...`.
  - Linux/macOS: native per-user config with `--data-dir` override.
- Input parity: keyboard/mouse mapping, cursors, fullscreen/windowed.
- Localisation: support all languages in `Resource.dll`.
- Acceptance: existing saves/config carry over; audio and settings match.

### M7 — Parity and polish

- Differential testing against the original under Wine/Xvfb.
- Fix visual/timing differences.
- Performance work.
- Platform-specific fixes.
- Acceptance: feature-complete, playable end to end.

### M8 — Improvements (post-1.0)

- Higher-quality renderer (`wgpu` backend).
- Better scaling/shaders.
- Quality-of-life improvements.
- Save format extensions.
- Modding hooks.

## Current priorities

1. **M3 — full XUI rendering.**
   The splash screen is only a small part of the UI. The main menu, options, journal,
   map, and dialogs all use XUI.
2. **M4 — one playable hidden-object level.**
   This is the first true gameplay vertical slice and will exercise most core systems.
3. **Asset format documentation.**
   `.STD`, `.MSE`, `.WSG`, `.TGL`, `.TRG`, `.XUI`, and the `Resource.dll` lookup scheme
   need dedicated docs under `docs/`.
4. **Differential test harness.**
   Automated screenshot/state comparison against the original.

## Definition of done for v1.0.0

- All original game modes playable.
- Original assets and save/config formats supported.
- Linux, Windows, and macOS builds on x86_64 and aarch64.
- No known gameplay/visual/audio regressions.
- Open-source code with user-supplied assets.