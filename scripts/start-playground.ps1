param(
  [string[]]$Features = @()
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run build:shared
pnpm run -F '@tauri-store/pinia' build

$ArgumentList = 'tauri dev'
foreach ($Feature in $Features) {
  $ArgumentList += " -f $Feature"
}

$WorkingDir = Get-Location |
  Select-Object -ExpandProperty Path |
  Join-Path -ChildPath 'examples/playground'

$Params = @{
  FilePath         = 'cargo'
  ArgumentList     = $ArgumentList
  WorkingDirectory = $WorkingDir
  NoNewWindow      = $true
  Wait             = $true
}

Start-Process @Params