<script lang="ts">
  import { Ext } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.9.0';

  // prettier-ignore
  const url = {
    hooks: 'https://tb.dev.br/tauri-store/reference/tauri-store/types/StoreHooks.html',
    storeId: 'https://docs.rs/tauri-store/0.9.0/tauri_store/struct.StoreId.html',
    watcherId: 'https://docs.rs/tauri-store/0.9.0/tauri_store/struct.WatcherId.html',
  };
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>Use different files for the stores when in development mode.</li>
    <li>
      Introduce <Ext href={url.storeId} code>StoreId</Ext> struct.
    </li>
    <li>
      Introduce <Ext href={url.storeId} code>WatcherId</Ext> struct.
    </li>
    <li>
      Remove deprecated <code>StoreOptions.onError</code>. Use
      <Ext href={url.hooks} code>StoreHooks.error</Ext> instead.
    </li>
  </ul>
</Changelog.BreakingChanges>
