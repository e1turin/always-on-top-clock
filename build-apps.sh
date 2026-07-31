#!/bin/bash
set -e

cargo build --release

BINARY_DIR="target/release"
APPS_DIR="target/apps"

rm -rf "$APPS_DIR"
mkdir -p "$APPS_DIR"

create_app() {
    local bin_name="$1"
    local app_name="$2"
    local bundle_id="$3"
    local app_dir="$APPS_DIR/$app_name.app"

    mkdir -p "$app_dir/Contents/MacOS"
    mkdir -p "$app_dir/Contents/Resources"

    cat > "$app_dir/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>$app_name</string>
    <key>CFBundleDisplayName</key>
    <string>$app_name</string>
    <key>CFBundleIdentifier</key>
    <string>$bundle_id</string>
    <key>CFBundleExecutable</key>
    <string>$bin_name</string>
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
PLIST

    cp "$BINARY_DIR/$bin_name" "$app_dir/Contents/MacOS/"
    chmod +x "$app_dir/Contents/MacOS/$bin_name"
    # The linker leaves binaries with an ad-hoc stub signature. Sign the bundle so
    # Gatekeeper does not treat it as damaged.
    codesign --force --deep --sign - "$app_dir"
    echo "Created $app_dir"
}

create_app "clock" "PiP Clock" "com.pip-clock.clock"
create_app "vertical" "PiP Vertical" "com.pip-clock.vertical"
create_app "pomodoro" "PiP Pomodoro" "com.pip-clock.pomodoro"

echo ""
echo "Done! Apps are in $APPS_DIR/"
echo "To install, drag the .app folders to /Applications/"
