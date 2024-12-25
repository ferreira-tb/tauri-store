$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

cargo run -p tauri-store-cli -- docs
pnpm run -F docs build

$WithNamespace = @('shared')

function Build-PackageDocs {
  param([string]$Name)

  if ($WithNamespace -contains $Name) {
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

  if ($SkipCrate -notcontains $Name) {
    Invoke-Expression "cargo +nightly doc -p $Name --no-deps"
  }
}

Get-ChildItem -Path './crates' -Directory |
  ForEach-Object { Build-CrateDocs -Name $_.Name }


Copy-Item -Path './target/doc' -Destination './docs/.vitepress/dist/rust-docs' -Recurse
