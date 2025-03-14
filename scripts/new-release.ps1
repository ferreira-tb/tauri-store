<# 
  .SYNOPSIS
  Create a new release on GitHub.
#>

param(
  [Alias('P', 'C', 'Crate')]
  [Parameter(Mandatory)]
  [string]$Package,

  [Alias('V')]
  [Parameter(Mandatory)]
  [semver]$Version
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$AllowedPackages = @(
  'tauri-store',
  '@tauri-store/pinia',
  '@tauri-store/svelte',
  '@tauri-store/valtio'
)

if ($AllowedPackages -notcontains $Package) {
  throw "Invalid package: $Package"
}

$Title = "$Package v$Version"
$Tag = $Title -replace '\s', '-'
$Repo = 'ferreira-tb/tauri-store'

$KebabVersion = $Version -replace '\.', '-'
$Changelog = "https://tb.dev.br/tauri-store/changelog/$Package#v$KebabVersion"
$Notes = @"
Please refer to the [changelog]($Changelog) for details.
"@

$Command = "gh release create `"$Tag`" -t `"$Title`" -n `"$($Notes.Trim())`" -R `"$Repo`""

Invoke-Expression $Command
