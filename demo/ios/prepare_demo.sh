#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
IOS_DIR="$ROOT_DIR/demo/ios"
XCFRAMEWORK_PATH="$ROOT_DIR/target/URRegistryFFI.xcframework"

if [ ! -d "$XCFRAMEWORK_PATH" ]; then
  echo "[iOS demo] URRegistryFFI.xcframework not found. Building..."
  (cd "$ROOT_DIR" && make generate_xcframework)
fi

echo "[iOS demo] Generating Xcode project..."
(cd "$IOS_DIR" && xcodegen generate)

echo "[iOS demo] Ready:"
echo "  $IOS_DIR/KeystoneFFIDemo.xcodeproj"

