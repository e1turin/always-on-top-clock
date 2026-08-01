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
./build-apps.py
```

Outputs `.app` bundles to `target/apps/`. Drag them to `/Applications/` to install.

## Gatekeeper

The bundles are signed ad-hoc during `build-apps.py`, so macOS won't treat them as damaged. However, because they are not signed with a Developer ID or notarized, the first time you open a downloaded app you'll get the "Apple cannot check it for malicious software" dialog. To open it:

- Right-click the app and choose **Open**, then confirm, or
- Remove the quarantine flag from the command line:

```bash
xattr -dr com.apple.quarantine /Applications/PiP\ Clock.app
```

For a fully seamless install (no warning at all), the app needs to be signed with an Apple Developer ID certificate and notarized — that requires a paid Apple Developer account.

## CI / Releases

The `Build` workflow runs on every push to `main`. It builds all apps and uploads the `.app` zips as a build artifact. It does not create releases.

To publish a release:

1. Tag a commit and push the tag:

   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. Once that build for tag succeeds, go to the **Actions** tab, open the `Release` workflow, and run it manually, entering the tag (e.g. `v1.0.0`) as input.

The release is versioned by the tag and attaches each app as a separate zip: `PiP Clock.zip`, `PiP Vertical.zip`, `PiP Pomodoro.zip`.

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
