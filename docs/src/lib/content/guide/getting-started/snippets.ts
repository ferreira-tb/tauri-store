import { kebabCase } from 'change-case';
import { snippet, snippetGroup } from '$lib/stores/snippet';

export const installCrate = snippet((metadata) => {
  return `
[dependencies]
${metadata.name} = "${metadata.version}"
`;
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
  return [
    {
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
    },
  ];
});
