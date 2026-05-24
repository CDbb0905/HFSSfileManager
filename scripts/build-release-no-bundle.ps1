$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot

Set-Location $root
$env:Path += ";" + "$env:USERPROFILE\.cargo\bin"

Write-Host "Installing/updating npm dependencies..."
npm.cmd install

Write-Host "Building release executable (no bundle)..."
npm.cmd run tauri build -- --no-bundle

Write-Host ""
Write-Host "Done. Executable path:"
Write-Host (Join-Path $root "src-tauri\target\release\hfss-file-manager.exe")
