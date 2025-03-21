$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run build:shared
pnpm run -F '@tauri-store/pinia' build

$WorkingDir = Get-Location |
  Select-Object -ExpandProperty Path |
  Join-Path -ChildPath 'examples/playground'

$Params = @{
  FilePath         = 'cargo'
  ArgumentList     = 'tauri dev'
  WorkingDirectory = $WorkingDir
  NoNewWindow      = $true
  Wait             = $true
}

Start-Process @Params