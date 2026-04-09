#!/usr/bin/env bash
# Build the game for WASM and prepare the web/ directory for serving.
#
# Prerequisites:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli
#
# Usage:
#   ./build-wasm.sh          # release build
#   ./build-wasm.sh --debug  # debug build (faster compile, larger output)
#
# Then serve the web/ directory, e.g.:
#   python3 -m http.server -d web 8080
#   # or: npx serve web

set -euo pipefail

PROFILE="release"
CARGO_FLAG="--release"

if [[ "${1:-}" == "--debug" ]]; then
  PROFILE="debug"
  CARGO_FLAG=""
fi

echo "==> Building game for wasm32-unknown-unknown ($PROFILE)..."
cargo build $CARGO_FLAG --target wasm32-unknown-unknown -p game

echo "==> Running wasm-bindgen..."
wasm-bindgen \
  --out-dir web \
  --out-name wormzone \
  --target web \
  "target/wasm32-unknown-unknown/$PROFILE/game.wasm"

# Copy assets next to the HTML so the Bevy AssetPlugin can find them.
echo "==> Syncing assets..."
rm -rf web/assets
cp -r assets web/assets

echo "==> Done! Serve the web/ directory to play in a browser."
echo "    Example: python3 -m http.server -d web 8080"
