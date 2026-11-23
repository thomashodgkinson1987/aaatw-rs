# Original binary analysis

This page records the facts needed to reverse-engineer the original game and to
regenerate the analysis environment. It is reference material for the port; it is
not a substitute for reading the code.

## Source files

The game directory (`/workspace`) contains:

- `AmazingAdventures2.exe` — the active Steam EXE. It is currently the
  splash-patched build (6 bytes changed).
- `AmazingAdventures2.exe.aaatw-orig` — the pristine backup of the stock EXE.
- `Resource.dll` — the 40 MB asset container.
- `bass.dll`, `config.txt`, `drm/`, `properties/`, `publisher.jpg`, etc.

The real game is embedded in `AmazingAdventures2.exe` as a PE overlay.

## Extracting `popcapgame1.exe`

Overlay location in `AmazingAdventures2.exe`:

- File offset: `0x17b1a2`
- Size: `0x165e00` (1,465,856 bytes)

```python
from pathlib import Path

src = Path("/workspace/AmazingAdventures2.exe.aaatw-orig")
b = src.read_bytes()
off = 0x17b1a2
size = 0x165e00
Path("/workspace/aaatw/popcapgame1.exe").write_bytes(b[off:off + size])
```

The overlay is written **verbatim** — it is not encrypted or compressed.

## Hashes

| File | SHA-256 |
|---|---|
| Pristine outer EXE | `275b8b278b3ec38657635bca3cab02ac411e85e63d24f06139eb5f9d5d1c00e3` |
| Patched outer EXE | `d920cde4b531d61b377b5feead659ac13e8cb11a88cd9622761c2ed4ea4eb833` |
| Pristine payload | `18e40b96c95cfd1c4f7ae9df5bc522b76fcb8cfc9bd20586a417604ac7a4569e` |
| Patched payload | `42edd1c477bf05c530d6eeee9754c27e3cb0e0a57df184e833b336defe978bee` |
| `Resource.dll` | `581c31ccae1d403cf89304aa530ea1a5d6d4d9c768563aa54deba11457464db8` |

## Inner PE layout

The extracted `popcapgame1.exe` is a 32-bit x86 PE:

- `ImageBase`: `0x400000`
- No ASLR (`DllCharacteristics = 0x8000`)
- Four sections:

| Section | VA | File offset |
|---|---|---|
| `.text` | `0x401000` | `0x000400` |
| `.rdata` | `0x517000` | `0x116200` |
| `.data` | `0x540000` | `0x13e800` |
| `.rsrc` | `0x558000` | `0x14a200` |

VA → file offset:

```text
.text:  fo = 0x000400 + (va - 0x401000)
.rdata: fo = 0x116200 + (va - 0x517000)
.data:  fo = 0x13E800 + (va - 0x540000)
.rsrc:  fo = 0x14A200 + (va - 0x558000)
```

The embedded overlay in `AmazingAdventures2.exe` uses:

```text
amz_offset = 0x17B1A2 + inner_offset
```

## Ghidra

Ghidra 12.2 is installed at `/opt/ghidra`.

Project:

```text
/workspace/aaatw-analysis/ghidra/aaatw
```

Headless import:

```sh
export JAVA_HOME=/usr/lib/jvm/default-runtime
/opt/ghidra/support/analyzeHeadless \
  /workspace/aaatw-analysis/ghidra aaatw \
  -import /workspace/aaatw/popcapgame1.exe
```

The helper script `/workspace/aaatw-analysis/scripts/DecompileSplash.java`
decompiles the key functions and writes:

```text
/workspace/aaatw-analysis/decompile-splash.txt
```

Re-run the post-script against the existing project:

```sh
/opt/ghidra/support/analyzeHeadless \
  /workspace/aaatw-analysis/ghidra aaatw \
  -process popcapgame1.exe -noanalysis \
  -scriptPath /workspace/aaatw-analysis/scripts \
  -postScript DecompileSplash.java
```

## Key addresses (inner game, image base `0x400000`)

| Address | Meaning |
|---|---|
| `0x405770` | `Application::Init()` |
| `0x40c000` | Splash/XUI loader |
| `0x406a30` | Per-frame front-end update / splash state dispatch |
| `0x407150` | `setState()` |
| `0x407780` | `getState()` |
| `0x407850` | State transition |
| `0x4065f0` | Tween builder |
| `0x406730` | Tween callback (forward) |
| `0x406910` | Tween callback (reverse) |
| `0x4090f0` | Select panel / menu preparation |
| `0x473be0` | VFS file open (`<scheme>:<path>`) |
| `0x481490` | Texture node resolver / standard-name check |
| `0x4934f0` | `_wcsstr` wide-string search |

Timing constants (32-bit floats in `.rdata`):

| Address | Value | Meaning |
|---|---|---|
| `0x5279e8` | 1.25 s | Splash transition/dwell |
| `0x52e148` | 1.75 s | State 4 → 5 |
| `0x52e0c0` | 2.5 s | State 5 / title |
| `0x52dff8` | 0.5 s | Pre-fade / hold |

UTF-16 strings:

| Address | Value |
|---|---|
| `0x52d428` | `std:distributor.jpg` |
| `0x52d450` | `std:publisher.jpg` |

## Asset resolution

The original VFS treats URIs as follows:

- **`std:<path>`** is a loose file relative to the game directory. It is **not**
  a `Resource.dll` lookup.
- **`<path>` without a scheme** is loaded from `Resource.dll` as a `RAWDATA`
  resource, using the uppercased asset name.

Confirmed by tracing the original:

- `std:publisher.jpg` opens the loose `publisher.jpg`.
- `std:distributor.jpg` opens the loose `distributor.jpg` if present.
- `spintopsplash.jpg` loads `SPINTOPSPLASH.JPG` from `Resource.dll`.
- `title_background.jpg` loads `TITLE_BACKGROUND.JPG` from `Resource.dll`.
- `std:distributormm.png` is used by the main menu, not the splash.

`Resource.dll` resource names are stored under the `RAWDATA` type. The name table
and extraction tool are in `aaatw-extract`.

## The game stack

- **SDA Game Framework** (`gfSDL`, `gfvfs.cpp`, © SDA Software Associates Inc.).
- **SDL 1.2**, statically linked.
- **DirectDraw** rendering only; no Direct3D.
- **BASS** audio via `bass.dll`.
- **Lua** is used by the outer DRM wrapper, not the inner game.
- The front-end source path string is `.\src\aaApplication.cpp`.
- The product/registry name is `Amazing Adventures Around The World`.

## Reference material

- `/workspace/amazing-adventures-wiki` — game reference wiki.
- `/workspace/amazing-adventures-toolkit` — companion patching toolkit.
- `docs/architecture.md` — port crate layout.
- `docs/splash.md` — splash asset/timing findings.
- `docs/testing.md` — Wine/Xvfb comparison workflow.