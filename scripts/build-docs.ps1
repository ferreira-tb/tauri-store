$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run build:shared
pnpm run sync
pnpm run -F docs build

function Build-PackageDocs {
  param([string]$Name)

  if ($Name.StartsWith('plugin-')) {
    $Name = $Name.Substring(7)
  }

  if ($Name -ne 'tauri-store') {
    $Name = "@tauri-store/$Name"
  }

  Invoke-Expression "pnpm run -F $Name typedoc"
}

Get-ChildItem -Path './packages' -Directory |
  ForEach-Object { Build-PackageDocs -Name $_.Name }
