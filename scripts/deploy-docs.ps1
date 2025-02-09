$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

sudo apt-get update
sudo apt-get -y install libgtk-3-dev libwebkit2gtk-4.1-dev

pnpm install
pnpm run codegen
pnpm run build
pnpm run build:docs

touch docs/dist/.nojekyll
