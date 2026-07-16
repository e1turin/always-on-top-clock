# PiP Clock

Native macOS always-on-top widgets built with **GPUI** (Zed's GPU-accelerated UI framework).

## Apps

| App | Binary | Window | Description |
|-----|--------|--------|-------------|
| **PiP Clock** | `clock` | 300×300 | `HH:MM:SS` digital clock |
| **PiP Vertical** | `vertical` | 300×300 | Hours over minutes in large type |
| **PiP Pomodoro** | `pomodoro` | 300×300 | 25/5/15 min timer with session tracking |

All windows stay above everything (including full-screen apps) using `WindowKind::PopUp` and are draggable.

## Keyboard Shortcuts

| Key | Clock / Vertical | Pomodoro |
|-----|-----------------|----------|
| `T` | Toggle black/white theme | Toggle black/white theme |
| `Space` | — | Play / Pause |
| `S` | — | Skip phase |
| `R` | — | Reset |

## Building

Requires macOS with Xcode (for Metal shader compilation) and a local clone of [Zed](https://github.com/zed-industries/zed) one directory above:

```
parent/
├── zed/                  # git clone https://github.com/zed-industries/zed
└── pip-clock/            # this repo
```

```bash
cargo build --release
```

## Running

```bash
cargo run --release --bin clock
cargo run --release --bin vertical
cargo run --release --bin pomodoro
```

## Creating .app Bundles

```bash
./build-apps.sh
```

Outputs `.app` bundles to `target/apps/`. Drag them to `/Applications/` to install.

## CI / Releases

GitHub Actions builds on every push to `main`. To create a release with downloadable `.app` zips:

```bash
git tag v1.0.0
git push origin v1.0.0
```

## Architecture

```
src/
├── lib.rs              # Shared Theme enum (Dark / Light)
└── bin/
    ├── clock.rs        # HH:MM:SS clock, ticks every second
    ├── vertical.rs     # Large hours/minutes, ticks every second
    └── pomodoro.rs     # Phase-based timer with UI controls
```

Each binary is a standalone GPUI application using `gpui_platform::application()` as the entry point. Windows are created with `WindowKind::PopUp` which sets `NSPopUpMenuWindowLevel` (101) and `NSWindowCollectionBehaviorCanJoinAllSpaces` — keeping the widget visible above all windows across all Spaces.
