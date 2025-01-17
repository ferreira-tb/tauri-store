import { snippetGroup } from '$stores/snippet';
import { pascalCase, snakeCase } from 'change-case';

export const customDirectory = snippetGroup((metadata) => {
  return {
    id: 'custom-directory',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .path("/path/to/custom/directory")
  .build();
  `,
  };
});

export const setCollectionPath = snippetGroup((metadata) => {
  const title = snakeCase(metadata.title!);
  const pascalTitle = pascalCase(metadata.title ?? '');
  return [
    {
      id: 'set-collection-path-ts',
      label: 'JavaScript',
      lang: 'typescript',
      value: `
import { set${pascalTitle}Path } from '${metadata.name}';

await set${pascalTitle}Path('/path/to/new/directory');
      `,
    },
    {
      id: 'set-collection-path-rs',
      label: 'Rust',
      lang: 'rust',
      value: `
use ${snakeCase(metadata.name)}::ManagerExt;

manager.${title}().set_path("/path/to/new/directory");
`,
    },
  ];
});
