#!/usr/bin/env bash
set -e

echo "=== [Vercel Build] Setting up Rust & Dioxus Toolchain ==="

# 1. Install Rust if not already in container
if ! command -v rustc &> /dev/null; then
  echo "Installing Rust toolchain..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile minimal
fi

export PATH="$HOME/.cargo/bin:$PATH"

# 2. Add wasm32 target
echo "Adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# 3. Setup Dioxus CLI (downloads precompiled binary in seconds)
DX_VERSION="0.7.10"
if ! command -v dx &> /dev/null || [ "$(dx --version 2>/dev/null | awk '{print $2}')" != "$DX_VERSION" ]; then
  echo "Downloading precompiled dioxus-cli v${DX_VERSION}..."
  curl -sSL "https://github.com/DioxusLabs/dioxus/releases/download/v${DX_VERSION}/dx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz
  chmod +x dx
  export PATH="$PWD:$PATH"
fi

echo "Rust version: $(rustc --version)"
echo "Dioxus version: $(dx --version)"

# 4. Clean previous bundle outputs if present
rm -rf hotkey_web/dist hotkey_web/target/dx

# 5. Compile WebAssembly binary & bundle web assets
echo "=== [Vercel Build] Compiling Hotkey Web to WebAssembly (Release) ==="
cd hotkey_web
dx bundle --platform web --release
cd ..

# 6. Stage dist/ for Vercel deployment
echo "=== [Vercel Build] Staging dist/ ==="
rm -rf dist
mkdir -p dist
if [ -d "hotkey_web/dist/public" ]; then
  cp -rf hotkey_web/dist/public/* dist/
elif [ -d "hotkey_web/target/dx/hotkey_web/release/web/public" ]; then
  cp -rf hotkey_web/target/dx/hotkey_web/release/web/public/* dist/
fi

touch dist/.nojekyll

echo "=== [Vercel Build] Successfully built and ready for deployment! ==="
