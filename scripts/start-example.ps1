<# 
  .SYNOPSIS
  Build and run the specified example.
#>

param(
  [string]$Example,
  [string[]]$Features = @()
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (-not $Example -or $Example -eq 'random') {
  $Examples = Get-ChildItem -Path './examples' -Directory -Exclude 'assets' |
    Select-Object -ExpandProperty Name

  $Example = Get-Random -InputObject $Examples
}

Write-Host "starting example: $($Example.ToUpper())"

pnpm run build:shared

$ArgumentList = 'tauri dev'
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
