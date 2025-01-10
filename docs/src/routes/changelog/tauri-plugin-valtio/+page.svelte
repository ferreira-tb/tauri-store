<script lang="ts">
  import { Breadcrumb } from '$lib/components/breadcrumb';
  import { Changelog, Container } from '$lib/components/container';
  import * as Valtio from '$lib/content/changelog/tauri-plugin-valtio';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-plugin-valtio" />
</svelte:head>

<Breadcrumb current="tauri-plugin-valtio" parents={['Changelog']} />

<Container title="Changelog" level={1} />

<Changelog.Next />

<Valtio.V0x />
