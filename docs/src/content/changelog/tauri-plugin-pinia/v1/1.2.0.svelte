<script lang="ts">
  import { Ext } from '$components/link';
  import { Changelog } from '$components/container';

  const version = '1.2.0';

  // prettier-ignore
  const url = {
    hooks: 'https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/interfaces/StoreHooks.html',
    onError: 'https://tb.dev.br/tauri-store/reference/tauri-plugin-pinia/interfaces/StoreFrontendOptions.html#onerror',
  };
</script>

<Changelog.Version {version} />

<Changelog.Features {version}>
  <ul>
    <li>Deprecate <Ext href={url.onError} code>StoreFrontendOptions.onError</Ext>.</li>
    <li>Add <Ext href={url.hooks} code>StoreHooks</Ext> to the plugin options.</li>
  </ul>
</Changelog.Features>
