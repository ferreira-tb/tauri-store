<script lang="ts">
  import { Breadcrumb } from '$lib/components/breadcrumb';
  import { Changelog, Container } from '$lib/components/container';
  import * as Pinia from '$lib/content/changelog/tauri-plugin-pinia';
</script>

<svelte:head>
  <title>Changelog | tauri-store</title>
  <meta name="description" content="Changelog for tauri-plugin-pinia" />
</svelte:head>

<Breadcrumb current="tauri-plugin-pinia" parents={['Changelog']} />

<Container title="Changelog" level={1} />

<Changelog.Next />

<Pinia.V1x />
<Pinia.V0x />
