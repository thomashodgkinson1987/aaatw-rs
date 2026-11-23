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

## Reference addresses (inner game, image base `0x400000`)

| Address | Meaning |
|---|---|
| `0x40c000` | Splash/XUI loader. Loads `splashscreens.xui`, resolves the five nodes, and centres the distributor/publisher textures. |
| `0x406a30` | Per-frame front-end update and splash state dispatch. |
| `0x407850` | State transition. Records the old state, sets the new state, and builds the fade tween. |
| `0x4065f0` | Tween builder. Converts seconds to milliseconds and registers the per-frame callback. |
| `0x473be0` | VFS file open. Parses `<scheme>:<path>`, matches a VFS mount, and opens either a loose file or an `RAWDATA` resource. |
| `0x481490` | Texture node resolver. Calls the texture loader and checks the standard splash names. |
| `0x52d428` | UTF-16 `std:distributor.jpg`. |
| `0x52d450` | UTF-16 `std:publisher.jpg`. |
| `0x5279e8` | `1.25` seconds. |
| `0x52e148` | `1.75` seconds. |
| `0x52e0c0` | `2.5` seconds. |

The VFS code is the reason `std:publisher.jpg` loads the loose `publisher.jpg`
while `spintopsplash.jpg` loads `SPINTOPSPLASH.JPG` from `Resource.dll`.
