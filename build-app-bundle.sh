#!/bin/bash
set -e

# Build the app
cargo build --release -p jogger-macos

# Create app bundle structure
APP_NAME="Jogger"
ICON_NAME="Gerald"
BUNDLE_DIR="target/release/${APP_NAME}.app"
CONTENTS_DIR="${BUNDLE_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

rm -rf "${BUNDLE_DIR}"
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy binary
cp target/release/jogger-macos "${MACOS_DIR}/${APP_NAME}"

# Create app icon (Gerald!) and convert to .icns for macOS bundle
ICONSET_DIR="$(mktemp -d)/${ICON_NAME}.iconset"
BASE_ICON_PNG="$(mktemp -t jogger-icon).png"
mkdir -p "${ICONSET_DIR}"

swift - "$BASE_ICON_PNG" <<'SWIFT'
import AppKit

let outputPath = CommandLine.arguments[1]
let size = NSSize(width: 1024, height: 1024)

let image = NSImage(size: size)
image.lockFocus()

NSColor.clear.setFill()
NSRect(origin: .zero, size: size).fill()

let text = "🏃‍♂️"
let paragraph = NSMutableParagraphStyle()
paragraph.alignment = .center

let attributes: [NSAttributedString.Key: Any] = [
    .font: NSFont.systemFont(ofSize: 760),
    .paragraphStyle: paragraph
]

let textRect = NSRect(x: 0, y: 150, width: 1024, height: 760)
text.draw(in: textRect, withAttributes: attributes)

image.unlockFocus()

if let tiff = image.tiffRepresentation,
   let rep = NSBitmapImageRep(data: tiff),
   let png = rep.representation(using: .png, properties: [:]) {
    try png.write(to: URL(fileURLWithPath: outputPath))
} else {
    fputs("Failed to create base PNG icon\n", stderr)
    exit(1)
}
SWIFT

for size in 16 32 128 256 512; do
  sips -z "${size}" "${size}" "$BASE_ICON_PNG" --out "${ICONSET_DIR}/icon_${size}x${size}.png" >/dev/null
done

for size in 16 32 128 256 512; do
  size2x=$((size * 2))
  sips -z "${size2x}" "${size2x}" "$BASE_ICON_PNG" --out "${ICONSET_DIR}/icon_${size}x${size}@2x.png" >/dev/null
done

iconutil -c icns "${ICONSET_DIR}" -o "${RESOURCES_DIR}/${ICON_NAME}.icns"
rm -rf "${ICONSET_DIR}" "$BASE_ICON_PNG"

# Create Info.plist
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.jogger.macos</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>${ICON_NAME}</string>
    <key>CFBundleVersion</key>
    <string>0.3.4</string>
    <key>CFBundleShortVersionString</key>
    <string>0.3.4</string>
    <key>LSUIElement</key>
    <true/>
</dict>
</plist>
EOF

echo "✅ App bundle created at ${BUNDLE_DIR}"
echo "Run with: open ${BUNDLE_DIR}"
