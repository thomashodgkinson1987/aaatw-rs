# Testing

This page describes the local test workflow. The container has no physical
display or GPU, so testing uses **Xvfb** and software rendering. Wine is used to
run the original Windows binaries for comparison.

## Build

```sh
cargo build --release -p aaatw-cli
```

The Linux binary is `target/release/aaatw`.
The Windows binary can be cross-built with:

```sh
CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc \
CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++ \
cargo build --release -p aaatw-cli --target x86_64-pc-windows-gnu
```

## Run the port under Xvfb

```sh
Xvfb :99 -screen 0 1280x1024x24 -nolisten tcp &
export DISPLAY=:99
cd /workspace
/workspace/aaatw-rs/target/release/aaatw --data-dir /workspace
```

The window should be titled **“Amazing Adventures Around the World”** and be
800×600.

## Capture at 30 fps

```sh
WID=$(DISPLAY=:99 xdotool search --sync --onlyvisible --name 'Amazing Adventures' | head -1)
eval "$(DISPLAY=:99 xdotool getwindowgeometry --shell "$WID")"
ffmpeg -y -loglevel error \
  -f x11grab -video_size "${WIDTH}x${HEIGHT}" -framerate 30 \
  -i ":99+${X},${Y}" -t 15 \
  -c:v libx264 -preset ultrafast -pix_fmt yuv420p \
  /workspace/aaatw-analysis/timing/capture.mp4
```

Extract frames:

```sh
mkdir -p /workspace/aaatw-analysis/timing/frames
ffmpeg -y -loglevel error -i /workspace/aaatw-analysis/timing/capture.mp4 \
  -fps_mode passthrough /workspace/aaatw-analysis/timing/frames/f%04d.png
```

## Measure frame brightness

```python
from PIL import Image
import glob

files = sorted(glob.glob("/workspace/aaatw-analysis/timing/frames/f*.png"))
previous = None
for index, path in enumerate(files, start=1):
    image = Image.open(path).convert("RGB")
    pixels = list(image.getdata())
    mean = sum((r * 299 + g * 587 + b * 114) // 1000 for r, g, b in pixels) / (len(pixels) * 255)
    if previous is None or abs(mean - previous) > 0.002:
        print(f"{index:4d} t={(index - 1) / 30.0:6.3f}s mean={mean:.5f}")
    previous = mean
```

A correct splash sequence shows **plateaus** at full opacity, then a fade through
black, then the next screen. A continuous crossfade with no repeated means is a
sign the hold/pre-delay is missing.

## Run the original for comparison

```sh
export DISPLAY=:99
export WINEPREFIX=/workspace/aaatw/prefix
export WINEDEBUG=-all
cd /workspace/aaatw
wine /workspace/aaatw/popcapgame1.exe
```

The original window is also titled **“Amazing Adventures Around the World”** and
is 800×600. Its first distributor splash is black because `distributor.jpg` is
not shipped. The publisher, SpinTop, and title background should match the
corresponding port frames.

## Run the Windows port under Wine

```sh
export DISPLAY=:99
export WINEPREFIX=/workspace/aaatw/prefix
export WINEDEBUG=-all
cd /workspace
wine /workspace/aaatw-rs/dist/aaatw-0.1.4-x86_64-pc-windows-gnu.exe \
  --data-dir 'Z:\workspace'
```

Wine may print `MESA-EGL`/`DRI3` warnings and audio-driver errors. These are
non-fatal for the splash test.

## Known-good splash timing

At 30 fps, the original shows:

| Stage | Approximate timing |
|---|---|
| Black distributor hold | 0.5 s |
| Publisher fade-in | 1.25 s |
| Publisher hold | 0.5 s |
| Fade to black | 1.25 s |
| SpinTop fade-in | 1.25 s |
| SpinTop hold | 0.5–0.9 s |
| Fade to black | 1.25 s |
| Title background fade-in | 1.25 s |
| Title background hold | 4.25 s (1.75 + 2.5) |
| Fade to main menu | 1.25 s |

The port currently matches this through the title background. The main menu is
not implemented yet.

## Test checklist

- [ ] Splash order: distributor (black) → publisher → SpinTop → title background.
- [ ] Each splash has a visible hold at full opacity.
- [ ] Each transition fades through black, not a direct crossfade.
- [ ] `--skip-splash` goes straight to the title screen.
- [ ] `aaatw-extract list /workspace/Resource.dll` lists 3,485 resources.
- [ ] `aaatw-extract extract /workspace/Resource.dll SPLASHSCREENS.XUI out.xui` works.
- [ ] Loose `publisher.jpg` is used for `std:publisher.jpg`.
- [ ] `SPINTOPSPLASH.JPG` and `TITLE_BACKGROUND.JPG` are loaded from `Resource.dll`.
- [ ] No game files or assets were modified.
- [ ] No asset files were copied into the repository.

## Notes

- The container has no physical display. `Xvfb` is required.
- `XDG_RUNTIME_DIR` warnings from SDL2 can be ignored; the X11 driver still works.
- The original game must be run from `/workspace/aaatw` so it finds `Resource.dll`
  and `bass.dll` next to the extracted EXE.
- The original game’s audio errors are non-fatal in the container.
- For differential testing, capture both the original and the port with the same
  Xvfb screen size and window geometry.