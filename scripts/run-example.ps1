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

$ArgumentList = "tauri dev -f $Example"
foreach ($Feature in $Features) {
  $ArgumentList += " -f $Feature"
}

$WorkingDir = Get-Location |
  Select-Object -ExpandProperty Path |
  Join-Path -ChildPath "examples/$Example"

$Params = @{
  FilePath         = 'cargo'
  ArgumentList     = $ArgumentList
  WorkingDirectory = $WorkingDir
  NoNewWindow      = $true
  Wait             = $true
}

Start-Process @Params
