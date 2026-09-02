# Builds the web bundle and stages it in dist/ for Vercel.
#
# Vercel watches this repository and deploys whatever lands on main.
# This script bundles the Dioxus web app and stages it in dist/ for
# previewing locally, CI builds, or deploying to Vercel.

#Requires -Version 5
$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

if (-not (Get-Command dx -ErrorAction SilentlyContinue)) {
    throw "dioxus-cli not found. Install it with: cargo binstall dioxus-cli --version 0.7.10"
}

# Clear previous bundle artifacts so stale hashed files don't accumulate
$stage = Join-Path $root 'hotkey_web/dist'
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }

$targetStage = Join-Path $root 'hotkey_web/target/dx/hotkey_web/release/web'
if (Test-Path $targetStage) { Remove-Item $targetStage -Recurse -Force }

Write-Host "Bundling Hotkeys web application with Dioxus CLI..." -ForegroundColor Cyan
Set-Location (Join-Path $root 'hotkey_web')
dx bundle --platform web --release
if ($LASTEXITCODE -ne 0) { throw "dx bundle failed" }
Set-Location $root

$bundle = Join-Path $root 'hotkey_web/dist/public'
if (-not (Test-Path $bundle)) {
    $bundle = Join-Path $targetStage 'public'
}
if (-not (Test-Path $bundle)) { throw "bundle not found at $bundle" }

$dist = Join-Path $root 'dist'
if (Test-Path $dist) {
    Get-ChildItem -Path $dist -Force | Remove-Item -Recurse -Force
} else {
    New-Item -ItemType Directory -Path $dist | Out-Null
}
Copy-Item -Path (Join-Path $bundle '*') -Destination $dist -Recurse
New-Item -ItemType File -Force -Path (Join-Path $dist '.nojekyll') | Out-Null


$size = (Get-ChildItem $dist -Recurse -File | Measure-Object -Property Length -Sum).Sum
Write-Host ("dist/ ready ({0:N1} MB)" -f ($size / 1MB)) -ForegroundColor Green
Write-Host "Preview with: python serve_web.py or npx serve dist" -ForegroundColor Yellow
Write-Host "Deploy with: vercel deploy --prod (or push to main)" -ForegroundColor Yellow
