<# 
  .SYNOPSIS
  Build and run the specified example.
#>

param(
  [string]$Example = 'pinia',
  [string[]]$Features = @()
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run build:shared

$ArgumentList = 'tauri dev'
foreach ($Feature in $Features) {
  $ArgumentList += " -f $Feature"
}

$Location = Get-Location
$WorkingDir = Join-Path -Path $Location.Path -ChildPath "examples/$Example"
$Params = @{
  FilePath         = 'cargo'
  ArgumentList     = $ArgumentList
  WorkingDirectory = $WorkingDir
  NoNewWindow      = $true
  Wait             = $true
}

Start-Process @Params
