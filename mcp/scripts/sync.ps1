$ErrorActionPreference = "Stop"
$mcp = Split-Path -Parent $PSScriptRoot
$root = Split-Path -Parent $mcp
$dest = Join-Path $mcp "prompts"
if (Test-Path $dest) { Remove-Item $dest -Recurse -Force }
New-Item -ItemType Directory -Path $dest | Out-Null
$langs = @("zh-CN", "en-US", "ja-JP", "ko-KR", "es-ES", "fr-FR", "de-DE", "pt-BR", "ru-RU")
foreach ($l in $langs) {
  $src = Join-Path $root "$l\prompts"
  $d = Join-Path $dest "$l\prompts"
  New-Item -ItemType Directory -Path $d | Out-Null
  Copy-Item (Join-Path $src "*") -Destination $d -Recurse -Force
}
Write-Host "prompts synced to mcp/prompts"
