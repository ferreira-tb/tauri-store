<script lang="ts">
  import { Breadcrumb } from '$components/breadcrumb';
  import * as Store from '$content/changelog/tauri-store';
  import { Changelog, Container } from '$components/container';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-store" />
</svelte:head>

<Breadcrumb current="tauri-store" parents={['Changelog']} />

<Container title="Changelog" level={1} />

<Changelog.Next>
  <ul>
    <li>Add <code>Store::save_on_exit</code> and <code>StoreOptions::save_on_exit</code>.</li>
  </ul>
</Changelog.Next>

<Store.V0x />
