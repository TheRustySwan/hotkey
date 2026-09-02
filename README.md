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
python serve_web.py
# or with PowerShell
./serve_web.ps1

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

The web application is configured for continuous deployment on **Vercel** just like the Schedule (`Scheduler`), Notes, and Markdown apps.

### How it Works

1. **Prebuilt Static Serving**: The repository includes a production-ready `dist/` bundle.
2. **Vercel Configuration**: [`vercel.json`](vercel.json) configures:
   - Output directory: `dist`
   - Build command: `bash build_vercel.sh` (or empty for instant serving)
   - Clean SPA rewrites to `/index.html`
   - Optimized caching and `application/wasm` MIME type headers
3. **Cloud Build Script**: [`build_vercel.sh`](build_vercel.sh) automatically installs Rust and the precompiled Dioxus CLI (`dx`) to build from source on Vercel if triggered.
4. **GitHub Actions CI/CD**: [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml) automatically runs checks, builds the web bundle on pushes to `main`, stages the fresh bundle into `dist/`, and commits it so Vercel can serve it immediately.

### Connecting to Vercel

1. Push your changes to GitHub:
   ```sh
   git push origin main
   ```
2. Go to the [Vercel Dashboard](https://vercel.com/new) and click **"Add New..."** → **"Project"**.
3. Import the `TheRustySwan/hotkey` repository.
4. Vercel will automatically detect [`vercel.json`](vercel.json):
   - **Framework Preset**: Other
   - **Root Directory**: `./`
   - **Build Command**: `bash build_vercel.sh` (or override with empty to serve prebuilt `dist`)
   - **Output Directory**: `dist`
5. Click **Deploy**. Every new push to `main` will automatically deploy your updates.
