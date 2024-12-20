$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run -F docs build

function Build-PackageDocs {
  param([string]$Name)

  if ($Name -eq 'shared') {
    $Name = '@tauri-store/shared'
  }

  Invoke-Expression "pnpm run -F $Name typedoc"
}

Get-ChildItem -Path './packages' -Directory |
  ForEach-Object { Build-PackageDocs -Name $_.Name }


$SkipCrate = @('tauri-store-cli', 'tauri-store-macros')

function Build-CrateDocs {
  param([string]$Name)

  if ($SkipCrate -notcontains $Name) {
    Invoke-Expression "cargo doc -p $Name --no-deps"
  }
}

Get-ChildItem -Path './crates' -Directory |
  ForEach-Object { Build-CrateDocs -Name $_.Name }


Copy-Item -Path './target/doc' -Destination './docs/.vitepress/dist/rust-docs' -Recurse
