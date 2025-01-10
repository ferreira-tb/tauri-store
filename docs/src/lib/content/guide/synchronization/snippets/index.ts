import { snakeCase } from 'change-case';
import { snippetGroup } from '$lib/stores/snippet';

export * from './sync-options';

export const syncDenylist = snippetGroup((metadata) => {
  return {
    id: 'sync-denylist',
    label: 'src-tauri/src/main.rs',
    lang: 'rust',
    value: `
${snakeCase(metadata.name)}::Builder::new()
  .sync_denylist(&["store-1", "store-2"])
  .build();
  `,
  };
});
