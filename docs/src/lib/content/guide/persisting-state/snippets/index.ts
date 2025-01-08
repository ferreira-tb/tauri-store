import { snakeCase } from 'change-case';
import { snippet, snippetGroup } from '$lib/stores/snippet';

export * from './save-on-change';

export const saveStores = snippetGroup((metadata) => {
  const title = snakeCase(metadata.title!);
  return [
    {
      id: 'save-stores-ts',
      label: 'JavaScript',
      lang: 'typescript',
      value: `
import { save, saveAll } from '${metadata.name}';

// Save a single store.
await save('my-store');

// Save some stores.
await save('store-1', 'store-2', 'store-3');

// Save all stores.
await saveAll();
      `,
    },
    {
      id: 'save-stores-rs',
      label: 'Rust',
      lang: 'rust',
      value: `
use ${snakeCase(metadata.name)}::ManagerExt;

// Here, "manager" represents any type that implements the "Manager" trait provided by Tauri.
// This includes "AppHandle", "Window", and "WebviewWindow".
// See: https://docs.rs/tauri/latest/tauri/trait.Manager.html

// Save a single store.
manager.${title}().save("my-store");

// Save some stores.
manager.${title}().save_some(&["my-store", "my-store-2"]);

// Save all stores.
manager.${title}().save_all();
      `,
    },
  ];
});

export const autosave = snippet((metadata) => {
  return `
use std::time::Duration;

// Save every five minutes.
tauri::Builder::default()
  .plugin(
    ${snakeCase(metadata.name)}::Builder::new()
      .autosave(Duration::from_secs(300))
      .build(),
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
  `;
});

export const customDirectory = snippet((metadata) => {
  return `
tauri::Builder::default()
  .plugin(
    ${snakeCase(metadata.name)}::Builder::new()
      .path("/path/to/custom/directory")
      .build(),
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
  `;
});
