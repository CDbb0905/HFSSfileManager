param(
  [switch]$Rebuild
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root "src-tauri\target\release\hfss-file-manager.exe"

if ($Rebuild -or -not (Test-Path $exe)) {
  Write-Host "Building release executable (no bundle)..."
  $env:Path += ";" + "$env:USERPROFILE\.cargo\bin"
  Set-Location $root
  npm.cmd run tauri build -- --no-bundle
}

if (-not (Test-Path $exe)) {
  throw "Executable not found: $exe"
}

Write-Host "Launching: $exe"
Start-Process -FilePath $exe
