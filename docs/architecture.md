# Architecture

## Overview

`aaatw-rs` is a standalone reimplementation of the inner `popcapgame1.exe` game. It
loads the original assets directly from the game directory and never modifies them.

```text
Resource.dll ──> aaatw-assets ──> aaatw-core (App, Surface) ──> aaatw-sdl2 ──> window
       │                              │
       └──────────────> aaatw-cli ────┘
```

## Crates

### `aaatw-core`

Platform-independent game state and rendering primitives.

- `Surface` is an RGBA8 CPU framebuffer with alpha-blending blits.
- `Renderer` is the drawing abstraction implemented by the SDL2 backend.
- `AssetProvider` abstracts asset lookup so the core can load textures by URI.
- `App` owns the current `Screen` and the loaded splash/title images.

### `aaatw-assets`

Read-only parsers for the original data:

- `ResourceDll` parses the PE resource directory and exposes resources by name.
- `XuiDocument` parses the game's XML-based XUI format.

### `aaatw-platform`

Platform-neutral input events and key types. The SDL2 backend maps native events into
these types.

### `aaatw-sdl2`

SDL2 window, input pump, and a software renderer that streams `Surface` pixels into
SDL textures.

### `aaatw-cli`

The `aaatw` binary. It locates `Resource.dll`, constructs the asset provider, and runs
the SDL2 backend.

### `aaatw-extract`

Developer CLI for listing and extracting `Resource.dll` resources.

## v0.1.0 scope

- 800x600 window.
- Splash sequence: distributor, publisher, SpinTop splash, title background.
- Title screen.
- Resource and XUI parsing for the splash screen definitions.
- No gameplay, audio, save/config, or input-driven menus yet.