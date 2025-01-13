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

<Changelog.Next />

<Store.V0x />
