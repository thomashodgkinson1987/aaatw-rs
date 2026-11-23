# Splash screens

## Asset resolution

`SPLASHSCREENS.XUI` defines four textures:

```xml
<texture id="spintopsplash_tex" uri="spintopsplash.jpg"/>
<texture id="distributor_tex"  uri="std:distributor.jpg"/>
<texture id="publisher_tex"    uri="std:publisher.jpg"/>
<texture id="title_tex"        uri="title_background.jpg"/>
```

The original game's VFS treats these URIs differently:

- **`std:<path>`** is a loose file relative to the game directory (the current
  working directory). It is not a `Resource.dll` lookup.
- **`<path>` without a scheme** is loaded from `Resource.dll` as a `RAWDATA`
  resource, using the uppercased asset name.

This was confirmed by tracing the original game:

- `std:publisher.jpg` opens the loose `publisher.jpg`.
- `std:distributor.jpg` opens the loose `distributor.jpg` if present.
- `spintopsplash.jpg` and `title_background.jpg` are loaded from `Resource.dll`
  as `SPINTOPSPLASH.JPG` and `TITLE_BACKGROUND.JPG`.
- `std:distributormm.png` is used by the main menu, not the splash.

`distributor.jpg` is not shipped with the game, so the original first splash is
black. There is **no** `drm/logo.jpg` fallback.

## Timing

The original splash state machine uses these durations:

| State | Texture | Duration |
|---|---|---|
| 1 | `std:distributor.jpg` | 1.25 s |
| 2 | `std:publisher.jpg` | 1.25 s |
| 3 | `spintopsplash.jpg` | 1.25 s |
| 4 | `title_background.jpg` | 1.75 s |
| 5 | `title_background.jpg` | 2.5 s |

The port uses a continuous crossfade between these states. The first state fades
in from black, matching the original's black distributor splash when
`distributor.jpg` is absent.
