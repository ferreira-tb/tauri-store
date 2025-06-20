param(
  [switch]$Ubuntu
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($Ubuntu) {
  sudo apt-get update
  sudo apt-get -y install libgtk-3-dev libwebkit2gtk-4.1-dev
}

cargo clippy --workspace
cargo test -p tauri-store --tests -- --test-threads=1
