<# 
  .SYNOPSIS
  Release script for the tauri-store repository.

  .PARAMETER Targets
  Targets to publish. If not specified, all targets will be published.

  .PARAMETER DryRun
  Perform a dry run.
#>

param(
  [string[]]$Targets = @(),
  [switch]$DryRun
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run codegen
pnpm run clippy
pnpm run eslint
pnpm run build

function Publish-Crate {
  param([string]$Name)

  if ($Targets.Count -gt 0 -and $Targets -notcontains $Name) {
    return
  }

  $command = "cargo publish -p $Name"
  if ($DryRun) {
    $command += ' --dry-run'
  }
  
  Invoke-Expression $command
}

$Crates = @(
  'tauri-store-macros',
  'tauri-store-utils',
  'tauri-store'
)

foreach ($Crate in $Crates) {
  Publish-Crate -Name $Crate
}

Get-ChildItem -Path './crates' -Directory -Exclude 'tauri-store*' |
  ForEach-Object { Publish-Crate -Name $_.Name }

function Publish-Package {
  param([string]$Name)

  if ($Targets.Count -gt 0 -and $Targets -notcontains $Name) {
    return
  }

  $command = "pnpm publish -F $Name"
  if ($DryRun) {
    $command += ' --dry-run'
  }

  if ($Name.StartsWith('@tauri-store')) {
    $command += ' --access public'
  }

  Invoke-Expression $command
}

Publish-Package -Name '@tauri-store/shared'
Get-ChildItem -Path './packages' -Directory -Exclude 'shared' |
  ForEach-Object { Publish-Package -Name $_.Name }
