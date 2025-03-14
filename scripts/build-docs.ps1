$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

pnpm run build:shared
pnpm run sync

cargo run -p tauri-store-cli -- docs
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


$SkipCrate = @(
  'tauri-store-cli',
  'tauri-store-macros'
)

function Build-CrateDocs {
  param([string]$Name)

  if ($Name.StartsWith('plugin-')) {
    $Name = "tauri-$Name"
  }

  if ($SkipCrate -notcontains $Name) {
    Invoke-Expression "cargo +nightly doc -p $Name --no-deps"
  }
}

Get-ChildItem -Path './crates' -Directory |
  ForEach-Object { Build-CrateDocs -Name $_.Name }


Copy-Item -Path './target/doc' -Destination './docs/dist/rust-docs' -Recurse
