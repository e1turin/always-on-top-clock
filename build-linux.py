#!/usr/bin/env python3
"""Build and package the Linux clock applications.

By default, this script compiles every binary in both dark and light variants,
stages them under target/linux-binaries/, and creates one ZIP archive per app
in target/linux-apps/. Run it on Linux so Cargo selects GPUI's Linux backend.
"""

import argparse
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent
BINARY_ROOT = PROJECT_ROOT / "target" / "linux-binaries"
APPS_DIR = PROJECT_ROOT / "target" / "linux-apps"
RELEASE_DIR = PROJECT_ROOT / "target" / "release"


@dataclass
class App:
    binary: str
    name: str


APPS = [
    App(binary="clock", name="Clock"),
    App(binary="vertical", name="Vertical Clock"),
    App(binary="pomodoro", name="Pomodoro Timer"),
    App(binary="stopwatch", name="Stopwatch"),
]

VARIANTS = (("dark", ""), ("light", " Light"))


def run(command: list[str]) -> None:
    subprocess.run(command, check=True, cwd=PROJECT_ROOT)


def build_binaries(binary_root: Path) -> None:
    for variant, _ in VARIANTS:
        command = ["cargo", "build", "--release", "--bins", "--locked"]
        if variant == "light":
            command.extend(["--features", "light-theme"])

        print(f"Building {variant} Linux binaries...")
        run(command)

        variant_dir = binary_root / variant
        if variant_dir.exists():
            shutil.rmtree(variant_dir)
        variant_dir.mkdir(parents=True)

        for app in APPS:
            source = RELEASE_DIR / app.binary
            if not source.is_file():
                raise FileNotFoundError(
                    f"Cargo did not produce the expected Linux binary: {source}"
                )
            destination = variant_dir / source.name
            shutil.copy2(source, destination)
            destination.chmod(0o755)


def create_archive(app: App, variant: str, name_suffix: str, binary_root: Path) -> None:
    display_name = f"{app.name}{name_suffix}"
    package_dir = APPS_DIR / display_name
    package_dir.mkdir(parents=True)

    source = binary_root / variant / app.binary
    if not source.is_file():
        raise FileNotFoundError(
            f"missing {variant} Linux binary: {source}; "
            "build it before running build-linux.py"
        )

    destination = package_dir / display_name
    shutil.copy2(source, destination)
    destination.chmod(0o755)
    shutil.copy2(PROJECT_ROOT / "LICENSE", package_dir / "LICENSE")

    archive = shutil.make_archive(
        str(APPS_DIR / f"{display_name}-Linux"),
        "zip",
        root_dir=package_dir,
    )
    print(f"Created {archive}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--binary-root",
        type=Path,
        default=BINARY_ROOT,
        help="directory used to stage dark/ and light/ Linux binaries",
    )
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="package already-staged binaries without running Cargo",
    )
    args = parser.parse_args()
    if sys.platform != "linux" and not args.skip_build:
        raise OSError("building Linux executables requires running on Linux")

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
            create_archive(app, variant, name_suffix, binary_root)

    print()
    print(f"Done! Linux ZIP archives are in {APPS_DIR}")


if __name__ == "__main__":
    try:
        main()
    except (FileNotFoundError, OSError, subprocess.CalledProcessError) as error:
        print(f"build-linux: error: {error}", file=sys.stderr)
        sys.exit(1)
