#!/usr/bin/env bash
# Deploy the WASM build to GitHub Pages (gh-pages branch).
#
# Usage: ./deploy-gh-pages.sh

set -euo pipefail

echo "==> Building WASM..."
./build-wasm.sh

echo "==> Deploying to gh-pages branch..."

# Save current branch
CURRENT_BRANCH=$(git branch --show-current)

# Create orphan gh-pages branch in a temp dir
TMPDIR=$(mktemp -d)
cp -r web/* "$TMPDIR/"

# Add .nojekyll to skip Jekyll processing (needed for _ prefixed files)
touch "$TMPDIR/.nojekyll"

cd "$TMPDIR"
git init
git checkout -b gh-pages
git add -A
git commit -m "Deploy WASM build to GitHub Pages"
git remote add origin git@github.com:marcoexexx/worm-vibe.git
git push origin gh-pages --force

cd -
rm -rf "$TMPDIR"

echo "==> Done! Enable GitHub Pages in repo settings:"
echo "    Settings → Pages → Source: Deploy from branch → gh-pages → / (root)"
echo ""
echo "    Your game will be at: https://marcoexexx.github.io/worm-vibe/"
