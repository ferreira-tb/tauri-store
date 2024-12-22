<# 
  .SYNOPSIS
  Update dependencies.

  .PARAMETER Include
  Avoid skipping the specified dependencies.
#>

param(
  [string[]]$Include = @()
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$Skip = @(
  '@tauri-apps/api',
  'pinia',
  'svelte',
  'tokio',
  'typescript'
)

$Command = 'miho update major -t'
foreach ($Item in $Skip) {
  if ($Include -notcontains $Item) {
    $Command += " -S $Item"
  }
}

Invoke-Expression $Command
