<script lang="ts">
  import { Ext } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.7.0';

  // prettier-ignore
  const url = {
    'Store::save_on_exit': 'https://docs.rs/tauri-store/0.7.0/tauri_store/struct.Store.html#method.save_on_exit',
    'StoreCollection::set_path': 'https://docs.rs/tauri-store/0.7.0/tauri_store/struct.StoreCollection.html#method.set_path',
    'StoreOptions::save_on_exit': 'https://docs.rs/tauri-store/0.7.0/tauri_store/struct.StoreOptions.html#structfield.save_on_exit',
  };
</script>

{#snippet ext(key: keyof typeof url)}
  <Ext href={url[key]} code>{key}</Ext>
{/snippet}

<Changelog.Version {version} />

<Changelog.Features {version}>
  <ul>
    <li>
      Add {@render ext('StoreCollection::set_path')}.
    </li>
    <li>
      Add {@render ext('Store::save_on_exit')} and {@render ext('StoreOptions::save_on_exit')}.
    </li>
  </ul>
</Changelog.Features>
