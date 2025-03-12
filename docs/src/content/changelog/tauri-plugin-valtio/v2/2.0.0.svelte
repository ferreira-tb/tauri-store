<script lang="ts">
  import { Ext, Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '2.0.0';

  // prettier-ignore
  const url = {
    hooks: 'https://tb.dev.br/tauri-store/reference/tauri-plugin-valtio/types/StoreHooks.html',
    tauriStore: 'https://docs.rs/tauri-store/0.9.0/tauri_store/',
  };
</script>

<Changelog.Version {version} />

<Changelog.BreakingChanges {version}>
  <ul>
    <li>
      Rename <code>getValtioPath</code> to <code>getStoreCollectionPath</code>.
    </li>
    <li>
      Rename <code>setValtioPath</code> to <code>setStoreCollectionPath</code>.
    </li>

    <li>
      Remove deprecated <code>StoreOptions.onError</code>. Use
      <Ext href={url.hooks} code>StoreHooks.error</Ext> instead.
    </li>

    <li>
      Update <Ext href={url.tauriStore} code>tauri-store</Ext> to
      <Link href="/tauri-store/changelog/tauri-store#v0-9-0" code>0.9.0</Link>.
    </li>
  </ul>
</Changelog.BreakingChanges>
