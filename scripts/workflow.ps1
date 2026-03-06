param(
  [ValidateSet("all", "check", "test", "build", "clean", "tauri-dev", "tauri-build")]
  [string]$Task = "all",
  [switch]$NoInstall,
  [switch]$DeepClean
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$srcTauri = Join-Path $repoRoot "src-tauri"
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"

function Step([string]$Name, [scriptblock]$Action) {
  Write-Host ""
  Write-Host "==> $Name" -ForegroundColor Cyan
  & $Action
}

function Exec([string]$Label, [scriptblock]$Command) {
  & $Command
  if ($LASTEXITCODE -ne 0) {
    throw "$Label failed with exit code $LASTEXITCODE"
  }
}

function Ensure-Tool([string]$Command, [string]$Hint) {
  if (-not (Get-Command $Command -ErrorAction SilentlyContinue)) {
    throw "$Command 不可用。$Hint"
  }
}

function Ensure-RustPath {
  if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    if (Test-Path (Join-Path $cargoBin "cargo.exe")) {
      $env:Path = "$cargoBin;$env:Path"
    }
  }

  Ensure-Tool "cargo" "请安装 Rust 并确保 $cargoBin 在 PATH。"
}

function Run-InRepo([string]$Name, [scriptblock]$Action) {
  Push-Location $repoRoot
  try {
    Step $Name $Action
  }
  finally {
    Pop-Location
  }
}

function Remove-IfExists([string]$Path) {
  if (Test-Path $Path) {
    Remove-Item -Recurse -Force $Path
    Write-Host "removed: $Path"
  }
}

function Ensure-Dependencies {
  Ensure-Tool "node" "请安装 Node.js 20+。"
  Ensure-Tool "npm.cmd" "请安装 npm。"

  if ($NoInstall) {
    return
  }

  if (-not (Test-Path (Join-Path $repoRoot "node_modules"))) {
    Run-InRepo "Install NPM Dependencies" { Exec "npm install" { npm.cmd install } }
  }
}

function Set-CargoTargetDir {
  # 将 Rust 构建输出统一收敛到 src-tauri/target，保持根目录整洁
  $env:CARGO_TARGET_DIR = Join-Path $srcTauri "target"
}

function Run-Check {
  Run-InRepo "Frontend Check" { Exec "npm run check" { npm.cmd run check } }
}

function Run-Tests {
  Run-InRepo "Frontend Tests" { Exec "npm run test -- --run" { npm.cmd run test -- --run } }

  Push-Location $srcTauri
  try {
    Step "Rust Tests" { Exec "cargo test" { cargo test } }
  }
  finally {
    Pop-Location
  }
}

function Run-Build {
  Run-InRepo "Frontend Build" { Exec "npm run build" { npm.cmd run build } }
}

function Run-TauriDev {
  Run-InRepo "Tauri Dev" { Exec "npm run tauri:dev" { npm.cmd run tauri:dev } }
}

function Run-TauriBuild {
  Run-InRepo "Tauri Build" { Exec "npm run tauri:build" { npm.cmd run tauri:build } }
}

function Run-Clean {
  Step "Clean Build Artifacts" {
    Remove-IfExists (Join-Path $repoRoot "dist")
    Remove-IfExists (Join-Path $repoRoot "build")
    Remove-IfExists (Join-Path $repoRoot ".svelte-kit")
    Remove-IfExists (Join-Path $repoRoot "target")
    Remove-IfExists (Join-Path $srcTauri "target")
  }

  if ($DeepClean) {
    Step "Deep Clean (Node Modules)" {
      Remove-IfExists (Join-Path $repoRoot "node_modules")
    }
  }
}

Ensure-Dependencies
Ensure-RustPath
Set-CargoTargetDir

switch ($Task) {
  "all" {
    Run-Check
    Run-Tests
    Run-Build
  }
  "check" { Run-Check }
  "test" { Run-Tests }
  "build" { Run-Build }
  "clean" { Run-Clean }
  "tauri-dev" { Run-TauriDev }
  "tauri-build" { Run-TauriBuild }
  default { throw "未知任务: $Task" }
}

Write-Host ""
Write-Host "Done: $Task" -ForegroundColor Green
