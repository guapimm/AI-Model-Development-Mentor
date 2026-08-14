$ErrorActionPreference = "Stop"
$cli = Split-Path -Parent $PSScriptRoot
& (Join-Path $cli "..\cli\scripts\sync-embed.ps1")
Push-Location $PSScriptRoot
try {
  cargo build --release
  if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
  $exe = Join-Path $PSScriptRoot "target\release\mentor.exe"
  if (Test-Path $exe) {
    New-Item -ItemType Directory -Force -Path (Join-Path $PSScriptRoot "dist") | Out-Null
    Copy-Item $exe (Join-Path $PSScriptRoot "dist\mentor-rust-windows-amd64.exe")
    Write-Host "built dist\mentor-rust-windows-amd64.exe"
  }
} finally {
  Pop-Location
}
