# Serve Web version of Hotkey App
$distDir = Join-Path $PSScriptRoot "dist"
if (-not (Test-Path (Join-Path $distDir "index.html"))) {
    Write-Host "Web build not found. Running build_web.ps1 first..." -ForegroundColor Yellow
    & (Join-Path $PSScriptRoot "build_web.ps1")
}

Write-Host "Starting web server on http://127.0.0.1:8080 ..." -ForegroundColor Cyan
python serve_web.py
