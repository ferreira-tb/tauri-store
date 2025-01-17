<script lang="ts">
  import { Ext, Link } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '0.2.0';

  // prettier-ignore
  const url = {
    saveOnExit:'https://tb.dev.br/tauri-store/reference/tauri-plugin-valtio/interfaces/StoreBackendOptions.html#saveonexit',
    setValtioPath: 'https://tb.dev.br/tauri-store/reference/tauri-plugin-valtio/functions/setValtioPath.html',
    tauriStore: 'https://docs.rs/tauri-store/0.7.0/tauri_store/',
  };
</script>

<Changelog.Version {version} />

<Changelog.Features {version}>
  <ul>
    <li>Add <Ext href={url.setValtioPath} code>setValtioPath</Ext>.</li>
    <li>Add <Ext href={url.saveOnExit} code>StoreBackendOptions.saveOnExit</Ext>.</li>
  </ul>
</Changelog.Features>

<Changelog.Dependencies {version}>
  <ul>
    <li>
      Update <Ext href={url.tauriStore} code>tauri-store</Ext> to
      <Link href="/tauri-store/changelog/tauri-store#v0-7-0" code>0.7.0</Link>.
    </li>
  </ul>
</Changelog.Dependencies>
