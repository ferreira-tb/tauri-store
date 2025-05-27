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
  '@tauri-store/valtio',
  '@tauri-store/vue',
  '@tauri-store/zustand'
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

$Changelog = 'https://tb.dev.br/tauri-store'
if ($Package -eq 'tauri-store') {
  $Changelog += '/changelog'
}
else {
  $Param = $Package -replace '@tauri-store/', 'plugin-'
  $Changelog += "/$Param/changelog"
}


$Notes = @"
Please refer to the [changelog]($Changelog) for details.
"@

$Repo = 'ferreira-tb/tauri-store'
$Command = "gh release create `"$Tag`" -t `"$Title`" -n `"$($Notes.Trim())`" -R `"$Repo`""

Invoke-Expression $Command
