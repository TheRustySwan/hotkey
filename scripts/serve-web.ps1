# Serve Web version of Hotkey App
$root = Split-Path -Parent $PSScriptRoot
$distDir = Join-Path $root "dist"
if (-not (Test-Path (Join-Path $distDir "index.html"))) {
    Write-Host "Web build not found. Running build-web.ps1 first..." -ForegroundColor Yellow
    & (Join-Path $PSScriptRoot "build-web.ps1")
}

Write-Host "Starting web server on http://127.0.0.1:8080 ..." -ForegroundColor Cyan
python (Join-Path $PSScriptRoot "serve_web.py")
