#!/usr/bin/env python3
"""Build macOS .app bundles for the PiP apps.

Runs `cargo build --release`, then assembles an ad-hoc signed .app bundle
for each app into target/apps/.
"""

import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent
BINARY_DIR = PROJECT_ROOT / "target" / "release"
APPS_DIR = PROJECT_ROOT / "target" / "apps"


@dataclass
class App:
    binary: str
    name: str
    bundle_id: str


APPS = [
    App(binary="clock", name="PiP Clock", bundle_id="com.pip-clock.clock"),
    App(binary="vertical", name="PiP Vertical", bundle_id="com.pip-clock.vertical"),
    App(binary="pomodoro", name="PiP Pomodoro", bundle_id="com.pip-clock.pomodoro"),
]

INFO_PLIST_TEMPLATE = """\
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>{name}</string>
    <key>CFBundleDisplayName</key>
    <string>{name}</string>
    <key>CFBundleIdentifier</key>
    <string>{bundle_id}</string>
    <key>CFBundleExecutable</key>
    <string>{binary}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>13.0</string>
    <key>LSUIElement</key>
    <true/>
</dict>
</plist>
"""


def run(command: list[str], cwd: Path | None = None) -> None:
    subprocess.run(command, cwd=cwd, check=True)


def create_app(app: App) -> None:
    app_dir = APPS_DIR / f"{app.name}.app"
    macos_dir = app_dir / "Contents" / "MacOS"
    resources_dir = app_dir / "Contents" / "Resources"
    macos_dir.mkdir(parents=True, exist_ok=True)
    resources_dir.mkdir(parents=True, exist_ok=True)

    (app_dir / "Contents" / "Info.plist").write_text(
        INFO_PLIST_TEMPLATE.format(
            name=app.name, bundle_id=app.bundle_id, binary=app.binary
        ),
        encoding="utf-8",
    )

    source = BINARY_DIR / app.binary
    destination = macos_dir / app.binary
    shutil.copy2(source, destination)
    destination.chmod(0o755)

    # The linker leaves binaries with an ad-hoc stub signature. Sign the bundle so
    # Gatekeeper does not treat it as damaged.
    run(["codesign", "--force", "--deep", "--sign", "-", str(app_dir)])

    print(f"Created {app_dir}")


def main() -> None:
    run(["cargo", "build", "--release"], cwd=PROJECT_ROOT)

    if APPS_DIR.exists():
        shutil.rmtree(APPS_DIR)
    APPS_DIR.mkdir(parents=True)

    for app in APPS:
        create_app(app)

    print()
    print(f"Done! Apps are in {APPS_DIR}/")
    print("To install, drag the .app folders to /Applications/")


if __name__ == "__main__":
    try:
        main()
    except (OSError, subprocess.CalledProcessError) as error:
        print(f"build-apps: error: {error}", file=sys.stderr)
        sys.exit(1)
