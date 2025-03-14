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
if ($Package -ne 'tauri-store') {
  $Crate = $Package -replace '@tauri-store/', 'tauri-plugin-'
  $Tag = "$Crate-v$Version"
}

$Filename = $Package
if ($Package -ne 'tauri-store') {
  $Filename = $Package -replace '@tauri-store/', 'plugin-'
}

$Repo = 'ferreira-tb/tauri-store'
$Changelog = "https://github.com/$Repo/blob/main/changelogs/$Filename.md"
$Notes = @"
Please refer to the [changelog]($Changelog) for details.
"@

$Command = "gh release create `"$Tag`" -t `"$Title`" -n `"$($Notes.Trim())`" -R `"$Repo`""

Invoke-Expression $Command
