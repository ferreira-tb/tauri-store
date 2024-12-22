<# 
  .SYNOPSIS
  Release script for the tauri-store repository.

  .PARAMETER Target
  Targets to publish. If not specified, all targets will be published.
#>

param(
  [string[]]$Target = @(),
  [switch]$DryRun,
  [switch]$NoVerify,
  [switch]$OnlyCrate,
  [switch]$OnlyPackage
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run codegen
pnpm run clippy
pnpm run eslint
pnpm run build

function Publish-Crate {
  param([string]$Name)

  if ($OnlyPackage) {
    return
  }

  if ($Target.Count -eq 0 -or $Target -contains $Name) {
    $command = "cargo publish -p $Name"
    if ($DryRun) {
      $command += ' --dry-run'
    }
  
    if ($NoVerify) {
      $command += ' --no-verify'
    }
    
    Invoke-Expression $command
  }
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

  if ($OnlyCrate) {
    return
  }

  if ($Target.Count -eq 0 -or $Target -contains $Name) {
    $command = "pnpm publish -F $Name"
    if ($DryRun) {
      $command += ' --dry-run'
    }
  
    if ($Name.StartsWith('@tauri-store')) {
      $command += ' --access public'
    }
  
    Invoke-Expression $command
  }
}

Publish-Package -Name '@tauri-store/shared'
Get-ChildItem -Path './packages' -Directory -Exclude 'shared' |
  ForEach-Object { Publish-Package -Name $_.Name }
