<# 
  .SYNOPSIS
  Publish the crates and packages to the registry.

  .PARAMETER Target
  Targets to publish. If not specified, all targets will be published.

  .PARAMETER DryRun
  Perform a dry run to see what would be published.

  .PARAMETER Fast
  Skip codegen and linting.
#>

param(
  [string[]]$Target = @(),
  [switch]$DryRun,
  [switch]$Fast,
  [switch]$OnlyCrate,
  [switch]$OnlyPackage,
  [switch]$SkipCodegen,
  [switch]$SkipLint,
  [switch]$SkipTest
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not $SkipCodegen -and -not $Fast) {
  pnpm run codegen
  pnpm run format
}

if (-not $SkipLint -and -not $Fast) {
  pnpm run clippy
  pnpm run eslint
  pnpm run type-check
}

if (-not $SkipTest -and -not $Fast) {
  pnpm run test:crate
}

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
