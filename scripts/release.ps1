param(
  [switch]$DryRun
)

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true

pnpm run codegen
pnpm run clippy
pnpm run lint
pnpm run build

function Publish-Crate {
  param(
    [Parameter(Mandatory = $true)]
    [ValidateNotNullOrEmpty()]
    [string]$Name
  )

  $command = "cargo publish -p $Name"
  if ($DryRun) {
    $command += " --dry-run"
  }
  
  Invoke-Expression $command
}

$Crates = @(
  "tauri-store-macros",
  "tauri-store-utils",
  "tauri-store"
)

foreach ($Crate in $Crates) {
  Publish-Crate -Name $Crate
}

Get-ChildItem -Path "./crates" -Directory -Exclude "tauri-store*" |
  ForEach-Object { Publish-Crate -Name $_.Name }

function Publish-Package {
  param(
    [Parameter(Mandatory = $true)]
    [ValidateNotNullOrEmpty()]
    [string]$Name
  )

  $command = "pnpm publish -F $Name"
  if ($DryRun) {
    $command += " --dry-run"
  }

  if ($Name.StartsWith("@tauri-store")) {
    $command += " --access public"
  }

  Invoke-Expression $command
}

Publish-Package -Name "@tauri-store/shared"
Get-ChildItem -Path "./packages" -Directory -Exclude "shared" |
  ForEach-Object { Publish-Package -Name $_.Name }
