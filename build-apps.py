#!/usr/bin/env python3
"""Build, assemble, and sign the PiP Clock macOS applications.

By default, this script compiles every binary in both dark and light variants,
stages them under target/binaries/, and creates signed application bundles in
target/apps/. GPUI runtime shaders are enabled so the build does not require
the optional command-line Metal toolchain.
"""

import argparse
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent
BINARY_ROOT = PROJECT_ROOT / "target" / "binaries"
APPS_DIR = PROJECT_ROOT / "target" / "apps"
RELEASE_DIR = PROJECT_ROOT / "target" / "release"


@dataclass
class App:
    binary: str
    name: str
    bundle_id: str


APPS = [
    App(binary="clock", name="Clock", bundle_id="com.pip-clock.clock"),
    App(binary="vertical", name="Vertical Clock", bundle_id="com.pip-clock.vertical"),
    App(binary="pomodoro", name="Pomodoro Timer", bundle_id="com.pip-clock.pomodoro"),
    App(
        binary="stopwatch",
        name="Stopwatch",
        bundle_id="com.pip-clock.stopwatch",
    ),
]

VARIANTS = (("dark", ""), ("light", " Light"))
RUNTIME_SHADER_FEATURE = "gpui_platform/runtime_shaders"

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


def run(command: list[str]) -> None:
    subprocess.run(command, check=True, cwd=PROJECT_ROOT)


def build_binaries(binary_root: Path) -> None:
    for variant, _ in VARIANTS:
        features = [RUNTIME_SHADER_FEATURE]
        if variant == "light":
            features.append("light-theme")

        print(f"Building {variant} binaries...")
        run(
            [
                "cargo",
                "build",
                "--release",
                "--bins",
                "--features",
                ",".join(features),
            ]
        )

        variant_dir = binary_root / variant
        if variant_dir.exists():
            shutil.rmtree(variant_dir)
        variant_dir.mkdir(parents=True)

        for app in APPS:
            source = RELEASE_DIR / app.binary
            if not source.is_file():
                raise FileNotFoundError(
                    f"Cargo did not produce the expected binary: {source}"
                )
            destination = variant_dir / app.binary
            shutil.copy2(source, destination)
            destination.chmod(0o755)


def create_app(app: App, variant: str, name_suffix: str, binary_root: Path) -> None:
    display_name = f"{app.name}{name_suffix}"
    bundle_id = f"{app.bundle_id}.{variant}" if variant == "light" else app.bundle_id
    app_dir = APPS_DIR / f"{display_name}.app"
    macos_dir = app_dir / "Contents" / "MacOS"
    resources_dir = app_dir / "Contents" / "Resources"
    macos_dir.mkdir(parents=True, exist_ok=True)
    resources_dir.mkdir(parents=True, exist_ok=True)

    (app_dir / "Contents" / "Info.plist").write_text(
        INFO_PLIST_TEMPLATE.format(
            name=display_name, bundle_id=bundle_id, binary=app.binary
        ),
        encoding="utf-8",
    )

    source = binary_root / variant / app.binary
    if not source.is_file():
        raise FileNotFoundError(
            f"missing {variant} binary: {source}; build it before running build-apps.py"
        )
    destination = macos_dir / app.binary
    shutil.copy2(source, destination)
    destination.chmod(0o755)

    # The linker leaves binaries with an ad-hoc stub signature. Sign the bundle so
    # Gatekeeper does not treat it as damaged.
    run(["codesign", "--force", "--deep", "--sign", "-", str(app_dir)])

    print(f"Created {app_dir}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--binary-root",
        type=Path,
        default=BINARY_ROOT,
        help="directory used to stage dark/ and light/ binaries",
    )
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="assemble bundles from already-staged binaries without running Cargo",
    )
    args = parser.parse_args()
    binary_root = args.binary_root
    if not binary_root.is_absolute():
        binary_root = PROJECT_ROOT / binary_root

    if not args.skip_build:
        build_binaries(binary_root)

    if APPS_DIR.exists():
        shutil.rmtree(APPS_DIR)
    APPS_DIR.mkdir(parents=True)

    for variant, name_suffix in VARIANTS:
        for app in APPS:
            create_app(app, variant, name_suffix, binary_root)

    print()
    print(f"Done! Apps are in {APPS_DIR}/")
    print("To install, drag the .app folders to /Applications/")


if __name__ == "__main__":
    try:
        main()
    except (FileNotFoundError, OSError, subprocess.CalledProcessError) as error:
        print(f"build-apps: error: {error}", file=sys.stderr)
        sys.exit(1)
