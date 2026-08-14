$ErrorActionPreference = "Stop"
$cli = Split-Path -Parent $PSScriptRoot
$root = Split-Path -Parent $cli
$dest = Join-Path $cli "files"
if (Test-Path $dest) { Remove-Item $dest -Recurse -Force }
New-Item -ItemType Directory -Path $dest | Out-Null
$langs = @("zh-CN", "en-US", "ja-JP", "ko-KR", "es-ES", "fr-FR", "de-DE", "pt-BR", "ru-RU")
$modules = @("AGENTS.md", "security.md", "style.md", "workflow.md")
foreach ($l in $langs) {
  $src = Join-Path $root "$l\prompts"
  $d = Join-Path $dest $l
  New-Item -ItemType Directory -Path $d | Out-Null
  Copy-Item (Join-Path $src "AGENTS.md") (Join-Path $d "agent.md")
  foreach ($m in $modules) {
    if ($m -ne "AGENTS.md") {
      Copy-Item (Join-Path $src $m) (Join-Path $d $m)
    }
  }
  $complete = Get-ChildItem $src -Filter *.md | Where-Object { $_.Name -notin $modules } | Select-Object -First 1
  if ($complete) { Copy-Item $complete.FullName (Join-Path $d "complete.md") }
}
Write-Host "embed synced to cli/files"
