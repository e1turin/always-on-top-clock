# PiP Clock

A native macOS always-on-top clock widget built with **GPUI** (Zed's GPU-accelerated UI framework).

## Features

- Always-on-top floating window (using `WindowKind::Floating` which maps to `NSFloatingWindowLevel` on macOS)
- GPU-accelerated rendering via Metal
- Clean, minimal design
- Updates every second
- Keyboard shortcuts:
  - `Cmd+Q` - Quit
  - `Cmd+T` - Toggle always-on-top (requires window recreation)

## Building

```bash
# From the pip-clock directory
cargo build --release
```

## Running

```bash
cargo run --release
```

Or run the built binary:
```bash
./target/release/pip-clock
```

## Notes

- Requires macOS with Xcode command line tools installed
- Uses GPUI from the local Zed workspace (`../../zed/crates/gpui`)
- The `Floating` window kind on macOS uses `NSFloatingWindowLevel` which keeps the window above normal windows
- Window background is transparent with a subtle dark backdrop

## Architecture

- `Clock` - The clock view that renders time and date
- `ClockApp` - Main app state handling actions and window management
- Uses GPUI's immediate-mode rendering with retained entity state
- Time updates driven by `request_animation_frame` + 1-second timer