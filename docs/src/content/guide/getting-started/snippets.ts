import { snippetGroup } from '$stores/snippet';
import { kebabCase, snakeCase } from 'change-case';

export const installCrate = snippetGroup((metadata) => {
  return {
    id: 'install-crate',
    label: 'src-tauri/Cargo.toml',
    lang: 'toml',
    value: `
[dependencies]
${metadata.name} = "${metadata.version}"
      `,
  };
});

export const installPackage = snippetGroup((metadata) => {
  return [
    {
      label: 'npm',
      lang: 'shell',
      value: `npm install ${metadata.name}@${metadata.version}`,
    },
    {
      label: 'pnpm',
      lang: 'shell',
      value: `pnpm add ${metadata.name}@${metadata.version}`,
    },
    {
      label: 'yarn',
      lang: 'shell',
      value: `yarn add ${metadata.name}@${metadata.version}`,
    },
    {
      label: 'bun',
      lang: 'shell',
      value: `bun add ${metadata.name}@${metadata.version}`,
    },
  ];
});

export const capabilities = snippetGroup((metadata) => {
  const title = kebabCase(metadata.title!);
  return {
    id: 'capabilities',
    label: `src-tauri/capabilities/${title}.json`,
    lang: 'json',
    value: `
{
  "identifier": "${title}",
  "windows": ["*"],
  "permissions": ["${title}:default", "core:event:default"]
}
      `,
  };
});

export const pluginRegistration = snippetGroup((metadata) => {
  return {
    id: 'plugin-registration',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
tauri::Builder::default()
  .plugin(${snakeCase(metadata.name)}::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
      `,
  };
});
