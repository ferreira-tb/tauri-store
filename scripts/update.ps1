<# 
  .SYNOPSIS
  Update dependencies.

  .PARAMETER Include
  Avoid skipping the specified dependencies.

  .PARAMETER IncludeAll
  Do not skip any dependencies.
#>

param(
  [string[]]$Include = @(),
  [switch]$IncludeAll
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$Skip = @(
  '@tauri-apps/api',
  'pinia',
  'svelte',
  'tokio',
  'valtio',
  'vue'
)

$Command = 'miho update major -t'

if (-not $IncludeAll) {
  foreach ($Dependency in $Skip) {
    if ($Include -notcontains $Dependency) {
      $Command += " -S $Dependency"
    }
  }
}

Invoke-Expression $Command
