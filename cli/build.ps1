$ErrorActionPreference = "Stop"
$cli = Split-Path -Parent $PSScriptRoot
& (Join-Path $cli "scripts\sync-embed.ps1")

$targets = @(
  @{ OS = "windows"; ARCH = "amd64"; NAME = "mentor-windows-amd64.exe" },
  @{ OS = "linux";   ARCH = "amd64"; NAME = "mentor-linux-amd64" },
  @{ OS = "darwin";  ARCH = "amd64"; NAME = "mentor-darwin-amd64" },
  @{ OS = "darwin";  ARCH = "arm64"; NAME = "mentor-darwin-arm64" }
)
$env:GOOS = $null
$env:GOARCH = $null
Push-Location $cli
try {
  foreach ($t in $targets) {
    $env:GOOS = $t.OS
    $env:GOARCH = $t.ARCH
    $out = Join-Path $cli ("dist\" + $t.NAME)
    Write-Host "building $($t.OS)/$($t.ARCH) -> $out"
    go build -trimpath -ldflags "-s -w" -o $out .
  }
  $env:GOOS = "windows"
  $env:GOARCH = "amd64"
  go build -trimpath -ldflags "-s -w" -o (Join-Path $cli "dist\mentor.exe") .
  Write-Host "done. artifacts in cli/dist/"
} finally {
  Pop-Location
}
