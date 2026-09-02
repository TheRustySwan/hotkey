# Hotkey App

A searchable, keyboard-driven cheat sheet for software keyboard shortcuts, built with Rust. Available both as a terminal UI (Ratatui + Crossterm) and an interactive Web application (Dioxus WebAssembly).

Supports shortcuts for VS Code, Neovim, Tmux, Git, Windows, macOS, Linux, and more with instant fuzzy searching and category filters.

---

## Running Locally

### Terminal UI (TUI)

```sh
cargo run --release
```

### Web Application (Dioxus Web)

```sh
# Option 1: Serve prebuilt bundle
python ./scripts/serve_web.py
# or with PowerShell
./scripts/serve-web.ps1

# Option 2: Run with Dioxus live reload
cd hotkey_web
dx serve --platform web
```

---

## Building the Web App

The web application is compiled to WebAssembly using the Dioxus CLI (`dx`) and staged into the root `dist/` directory.

```powershell
./build_web.ps1
# or
./scripts/build-web.ps1
```

This compiles `hotkey_web` in release mode, bundles the WASM binary and JS glue, and stages the deployment-ready static files in `dist/`.

---

## Deploying to Vercel

The web application is configured for continuous deployment on **Vercel** just like the Schedule (`Scheduler`) app.

### How it Works

1. **GitHub Actions builds; Vercel serves**: Push to `main` and [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml) runs checks, compiles the WASM bundle with `dx`, and commits it to `dist/`.
2. **Pure Static Serving**: [`vercel.json`](vercel.json) sets an empty `buildCommand: ""` and points `outputDirectory` directly at `dist/`. Vercel simply uploads and serves the prebuilt bundle in seconds with zero cold-recompile overhead and zero serverless crashes.
3. **Optimized Headers & Routing**: [`vercel.json`](vercel.json) includes SPA rewrites to `/index.html`, immutable caching for static assets, and the `application/wasm` MIME type header.

### Connecting to Vercel

1. In the [Vercel Dashboard](https://vercel.com/new), click **"Add New..."** → **"Project"** and import the `TheRustySwan/hotkey` repository.
2. Vercel detects [`vercel.json`](vercel.json):
   - **Framework Preset**: Other
   - **Root Directory**: `./`
   - **Build Command**: (leave empty / disabled)
   - **Output Directory**: `dist`
3. Click **Deploy**. Vercel will immediately deploy the static `dist/` bundle. Every subsequent push to `main` will build and publish automatically.
